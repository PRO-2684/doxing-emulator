//! Doxee resolution from Telegram source context.

use super::{dox_impl::DoxReport, messages::BotError};
use frakti::{
    client_cyper::Bot,
    types::{ExternalReplyInfo, Message, User},
};

/// Parsed argument naming an explicit doxee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoxArg {
    /// No explicit doxee was provided.
    None,
    /// Explicit Telegram user ID.
    UserId(u64),
    // TODO: Chat/Channel ID as i64
    /// An explicit doxee argument was provided, but it is not valid.
    Invalid,
}

impl DoxArg {
    /// Parse an optional raw doxee argument.
    #[must_use]
    pub fn parse(raw: Option<&str>) -> Self {
        let Some(raw) = raw.map(str::trim).filter(|raw| !raw.is_empty()) else {
            return Self::None;
        };
        raw.parse().map_or(Self::Invalid, Self::UserId)
    }
}

/// Source context for resolving a doxee.
pub enum DoxeeSource {
    /// `/dox` command invocation.
    Command {
        /// Parsed command argument.
        arg: DoxArg,
        /// Telegram message containing the command.
        message: Message,
    },
    /// Inline query invocation.
    Inline {
        /// Parsed inline query.
        arg: DoxArg,
        /// User issuing the inline query.
        from: User,
    },
    /// Guest mention invocation.
    Guest {
        /// Parsed guest mention argument.
        arg: DoxArg,
        /// Telegram guest message.
        message: Message,
    },
    /// Private non-command message.
    PrivateMessage {
        /// Telegram private message.
        message: Message,
    },
}

impl DoxeeSource {
    /// Resolve this source context into a dox report or bot error.
    pub async fn resolve_with(self, bot: &Bot) -> Option<Result<DoxReport, BotError>> {
        Some(match self {
            Self::Command { arg, message } => {
                Box::pin(resolve_message_source(bot, arg, message)).await
            }
            Self::Guest { arg, message } => {
                Box::pin(resolve_guest_message_source(bot, arg, message)).await
            }
            Self::Inline { arg, from } => resolve_inline_source(bot, arg, from).await,
            Self::PrivateMessage { message } => resolve_private_message(bot, message).await,
        })
    }
}

async fn resolve_message_source(
    bot: &Bot,
    arg: DoxArg,
    msg: Message,
) -> Result<DoxReport, BotError> {
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

    let Some(doxer_report) =
        DoxReport::from_sender(from, sender_chat, sender_tag.or(author_signature))
    else {
        return Err(BotError::DoxerIdentificationFailed);
    };

    match arg {
        DoxArg::None => {
            resolve_implicit_doxee(bot, (reply_to_message, external_reply), doxer_report).await
        }
        DoxArg::UserId(user_id) => DoxReport::from_id(bot, user_id, Some(chat.id))
            .await
            .ok_or(BotError::DoxeeIdentificationFailed),
        DoxArg::Invalid => Err(BotError::NotUserId),
    }
}

async fn resolve_guest_message_source(
    bot: &Bot,
    arg: DoxArg,
    msg: Message,
) -> Result<DoxReport, BotError> {
    let Message {
        from,
        sender_chat,
        sender_tag,
        author_signature,
        reply_to_message,
        external_reply,
        chat,
        guest_bot_caller_user,
        guest_bot_caller_chat,
        ..
    } = msg;

    let Some(doxer_report) = DoxReport::from_sender(
        guest_bot_caller_user.or(from),
        guest_bot_caller_chat.or(sender_chat),
        sender_tag.or(author_signature),
    ) else {
        return Err(BotError::DoxerIdentificationFailed);
    };

    match arg {
        DoxArg::None => {
            resolve_implicit_doxee(bot, (reply_to_message, external_reply), doxer_report).await
        }
        DoxArg::UserId(user_id) => DoxReport::from_id(bot, user_id, Some(chat.id))
            .await
            .ok_or(BotError::DoxeeIdentificationFailed),
        DoxArg::Invalid => Err(BotError::NotUserId),
    }
}

async fn resolve_inline_source(bot: &Bot, arg: DoxArg, from: User) -> Result<DoxReport, BotError> {
    match arg {
        DoxArg::None => Ok(DoxReport::from_user(from).complete_full_info(bot).await),
        DoxArg::UserId(user_id) => DoxReport::from_id(bot, user_id, None)
            .await
            .ok_or(BotError::DoxeeIdentificationFailed),
        DoxArg::Invalid => Err(BotError::NotUserId),
    }
}

async fn resolve_private_message(bot: &Bot, msg: Message) -> Result<DoxReport, BotError> {
    let Message {
        from,
        sender_chat,
        forward_origin,
        ..
    } = msg;

    let Some(origin) = forward_origin else {
        return Err(BotError::Incomprehensible);
    };
    if from.is_none() && sender_chat.is_none() {
        return Err(BotError::DoxerIdentificationFailed);
    }
    DoxReport::from_origin(bot, *origin, None)
        .await
        .ok_or(BotError::InvalidOrigin)
}

async fn resolve_implicit_doxee(
    bot: &Bot,
    ctx: (Option<Box<Message>>, Option<Box<ExternalReplyInfo>>),
    doxer_report: DoxReport,
) -> Result<DoxReport, BotError> {
    if let Some(reply) = ctx.0 {
        resolve_reply(bot, *reply).await
    } else if let Some(external) = ctx.1 {
        Box::pin(DoxReport::from_external_reply(bot, *external))
            .await
            .ok_or(BotError::InvalidOrigin)
    } else {
        Ok(doxer_report.complete_full_info(bot).await)
    }
}

async fn resolve_reply(bot: &Bot, reply: Message) -> Result<DoxReport, BotError> {
    if let Some(origin) = reply.forward_origin {
        DoxReport::from_origin(bot, *origin, Some(reply.chat.id))
            .await
            .ok_or(BotError::InvalidOrigin)
    } else if let Some(external) = reply.external_reply {
        Box::pin(DoxReport::from_external_reply(bot, *external))
            .await
            .ok_or(BotError::InvalidOrigin)
    } else if let Some(report) = DoxReport::from_sender(
        reply.from,
        reply.sender_chat,
        reply.sender_tag.or(reply.author_signature),
    ) {
        Ok(report.complete_full_info(bot).await)
    } else {
        Err(BotError::DoxeeIdentificationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::DoxArg;

    #[test]
    fn dox_arg_parse_empty_as_none() {
        assert_eq!(DoxArg::parse(None), DoxArg::None);
        assert_eq!(DoxArg::parse(Some("")), DoxArg::None);
        assert_eq!(DoxArg::parse(Some("  ")), DoxArg::None);
    }

    #[test]
    fn dox_arg_parse_user_id() {
        assert_eq!(DoxArg::parse(Some(" 123 ")), DoxArg::UserId(123));
    }

    #[test]
    fn dox_arg_parse_invalid_text() {
        assert_eq!(DoxArg::parse(Some("123 extra")), DoxArg::Invalid);
        assert_eq!(DoxArg::parse(Some("-100123")), DoxArg::Invalid);
    }
}
