//! Module for handling guest messages.

use super::{
    doxee_resolution::{DoxArg, DoxeeSource},
    inline::create_article,
};
use frakti::{client_cyper::Bot, inline_mode::InlineQueryResultArticle, types::Message};
use log::info;

/// Handle guest messages.
pub async fn handle_guest_message(
    bot: &Bot,
    msg: Message,
    bot_username: &str,
) -> Option<InlineQueryResultArticle> {
    info!("Handling guest message: {:?}", msg.text);

    let arg = parse_guest_invocation(msg.text.as_deref(), bot_username)?;
    let source = DoxeeSource::Guest { arg, message: msg };
    let result = Box::pin(source.resolve_with(bot))
        .await
        .expect("guest mention resolution should always reply");
    let message = match result {
        Ok(report) => report.to_string(),
        Err(error) => error.to_string(),
    };

    Some(create_article(message, "Title", "Description"))
}

fn parse_guest_invocation(text: Option<&str>, bot_username: &str) -> Option<DoxArg> {
    let text = text?.trim();
    let rest = text.strip_prefix('@')?.strip_prefix(bot_username)?;
    let Some(first) = rest.chars().next() else {
        return Some(DoxArg::None);
    };
    if first.is_whitespace() {
        return Some(DoxArg::parse(Some(rest)));
    }
    if first.is_ascii_alphanumeric() || first == '_' {
        return None;
    }
    Some(DoxArg::Invalid)
}

#[cfg(test)]
mod tests {
    use super::parse_guest_invocation;
    use crate::doxee_resolution::DoxArg;

    #[test]
    fn guest_invocation_accepts_exact_mention() {
        let text = "@DoxingEmulatorBot".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            Some(DoxArg::None)
        );
    }

    #[test]
    fn guest_invocation_accepts_mention_with_user_id() {
        let text = " @DoxingEmulatorBot 123 ".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            Some(DoxArg::UserId(123))
        );
    }

    #[test]
    fn guest_invocation_rejects_non_token_mention() {
        let text = "@DoxingEmulatorBot123".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            None
        );
    }

    #[test]
    fn guest_invocation_ignores_non_leading_mention() {
        let text = "hello @DoxingEmulatorBot 123".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            None
        );
    }

    #[test]
    fn guest_invocation_ignores_other_bot() {
        let text = "@OtherBot 123".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            None
        );
    }

    #[test]
    fn guest_invocation_replies_to_invalid_mentioned_arg() {
        let text = "@DoxingEmulatorBot 123 extra".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            Some(DoxArg::Invalid)
        );
    }

    #[test]
    fn guest_invocation_replies_to_invalid_mentioned_punctuation() {
        let text = "@DoxingEmulatorBot, 123".to_string();
        assert_eq!(
            parse_guest_invocation(Some(&text), "DoxingEmulatorBot"),
            Some(DoxArg::Invalid)
        );
    }
}
