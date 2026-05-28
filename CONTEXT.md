# Doxing Emulator

A Telegram bot context for generating playful dox reports from Telegram users, chats, channels, and message origins.

## Language

**Doxer**:
The Telegram user, chat, or guest sender invoking the emulator.

**Doxee**:
The Telegram user, chat, channel, or message origin represented in a dox report.
_Avoid_: target, victim

**Dox report**:
The HTML response that summarizes known Telegram profile, chat, title, birthday, business location, and personal channel details for a doxee.
_Avoid_: response, output

**Guest mention**:
A guest-mode invocation whose text is exactly `@username` or `@username` followed by one doxee user ID, ignoring surrounding whitespace.

## Example Dialogue

Dev: "If a doxer replies with `/dox`, who is the doxee?"

Domain expert: "The replied-to sender is the doxee, unless the reply points to a forwarded or external origin."

Dev: "If a guest message mentions the bot without a user ID, who is the doxee?"

Domain expert: "Use the same implicit doxee as `/dox`: replied-to sender when present, otherwise the doxer."
