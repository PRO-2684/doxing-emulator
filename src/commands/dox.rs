//! The dox command.

use super::{Command, dox_impl::DoxReport};
use crate::messages::BotError;
use frakti::{
    client_cyper::Bot,
    types::{ExternalReplyInfo, Message},
};

/// The dox command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dox {
    pub doxee: Option<String>,
}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    #[allow(clippy::similar_names)]
    async fn execute(self, bot: &Bot, msg: Message, _username: &str) -> String {
        let Message {
            from,
            sender_chat,
            sender_tag,
            author_signature,
            reply_to_message,
            external_reply,
            chat,
            ..
        } = msg;

        // Reject users that the bot doesn't know
        let Some(doxer_report) =
            DoxReport::from_sender(from, sender_chat, sender_tag.or(author_signature))
        else {
            return BotError::DoxerIdentificationFailed.to_string();
        };

        // Resolve the doxee based on the context and arguments.
        let result = match self.doxee {
            None => {
                resolve_implicit_doxee(bot, (reply_to_message, external_reply), doxer_report).await
            }
            Some(raw) => resolve_explicit_doxee(bot, &raw, chat.id).await,
        };
        match result {
            Ok(report) => report.to_string(),
            Err(error) => error.to_string(),
        }
    }
}

/// Target provided in /dox command
async fn resolve_explicit_doxee(bot: &Bot, raw: &str, chat_id: i64) -> Result<DoxReport, BotError> {
    let user_id = raw.parse().map_err(|_| BotError::NotUserId)?;
    DoxReport::from_id(bot, user_id, Some(chat_id))
        .await
        .ok_or(BotError::DoxeeIdentificationFailed)
}

/// Target not provided in /dox command - infer from context (reply or external reply)
async fn resolve_implicit_doxee(
    bot: &Bot,
    ctx: (Option<Box<Message>>, Option<Box<ExternalReplyInfo>>),
    doxer_report: DoxReport,
) -> Result<DoxReport, BotError> {
    if let Some(reply) = ctx.0 {
        resolve_reply(bot, *reply).await
    } else if let Some(external) = ctx.1 {
        DoxReport::from_external_reply(bot, *external)
            .await
            .ok_or(BotError::InvalidOrigin)
    } else {
        Ok(doxer_report.complete_full_info(bot).await)
    }
}

/// Resolve doxee from a reply message.
async fn resolve_reply(bot: &Bot, reply: Message) -> Result<DoxReport, BotError> {
    if let Some(origin) = reply.forward_origin {
        // Forwarded messages - the doxee is the original sender, not the forwarder
        DoxReport::from_origin(bot, *origin, Some(reply.chat.id))
            .await
            .ok_or(BotError::InvalidOrigin)
    } else if let Some(external) = reply.external_reply {
        // External replies - the doxee is the sender of the original message, not the replier
        DoxReport::from_external_reply(bot, *external)
            .await
            .ok_or(BotError::InvalidOrigin)
    } else if let Some(report) = DoxReport::from_sender(
        reply.from,
        reply.sender_chat,
        reply.sender_tag.or(reply.author_signature),
    ) {
        // Regular replies - the doxee is the sender of the reply
        Ok(report.complete_full_info(bot).await)
    } else {
        Err(BotError::DoxeeIdentificationFailed)
    }
}
