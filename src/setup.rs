//! Set up related code.

use super::commands::LIST;
use anyhow::Result;
use frakti::{
    AsyncTelegramApi, Error,
    client_cyper::Bot,
    methods::{DeleteMyCommandsParams, SetMyCommandsParams, SetMyDefaultAdministratorRightsParams},
    response::MethodResponse,
    types::{BotCommand, ChatAdministratorRights},
};
use futures_util::FutureExt;

/// Set up commands.
pub async fn setup_commands(bot: &Bot) -> Result<(), Error> {
    // Deletes previous commands
    let delete_params = DeleteMyCommandsParams::builder().build();
    bot.delete_my_commands(&delete_params).await?;

    // Available commands
    let commands = LIST.map(|(trigger, help)| {
        BotCommand::builder()
            .command(trigger)
            .description(help)
            .build()
    });

    // Register commands
    let commands_param = SetMyCommandsParams::builder()
        .commands(commands.to_vec())
        .build();
    bot.set_my_commands(&commands_param).await?;

    Ok(())
}

/// Set up rights.
pub fn setup_rights(bot: &Bot) -> impl Future<Output = Result<(), Error>> {
    let rights_param = SetMyDefaultAdministratorRightsParams::builder()
        .rights(RECOMMENDED_ADMIN_RIGHTS)
        .build();
    // bot.set_my_default_administrator_rights requires a reference, so we use low-level request method to avoid borrowing issues.
    bot.request("setMyDefaultAdministratorRights", Some(rights_param))
        .map(|result: Result<MethodResponse<bool>, Error>| result.map(|_| ()))
}

/// Recommended admin rights for the bot. (No privilege required)
const RECOMMENDED_ADMIN_RIGHTS: ChatAdministratorRights = ChatAdministratorRights {
    is_anonymous: false,
    can_manage_chat: false,
    can_delete_messages: false,
    can_manage_video_chats: false,
    can_restrict_members: false,
    can_promote_members: false,
    can_change_info: false,
    can_invite_users: false,
    can_post_messages: None,
    can_edit_messages: None,
    can_pin_messages: None,
    can_post_stories: None,
    can_edit_stories: None,
    can_delete_stories: None,
    can_manage_topics: None,
    can_manage_direct_messages: Some(false),
    can_manage_tags: Some(false),
};
