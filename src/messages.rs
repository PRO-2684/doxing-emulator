//! Static HTML bot error messages.

use std::fmt;

/// Built-in HTML error reply message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BotError {
    /// Failed to identify the command invoker.
    DoxerIdentificationFailed,
    /// Failed to identify the dox target.
    DoxeeIdentificationFailed,
    /// The message origin cannot be used as a dox target.
    InvalidOrigin,
    /// Provided argument is not a user ID.
    NotUserId,
    /// Non-command message cannot be understood.
    Incomprehensible,
}

impl BotError {
    /// Return the error message HTML.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoxerIdentificationFailed => {
                include_str!("messages/doxer-identification-failed.html")
            }
            Self::DoxeeIdentificationFailed => {
                include_str!("messages/doxee-identification-failed.html")
            }
            Self::InvalidOrigin => include_str!("messages/invalid-origin.html"),
            Self::NotUserId => include_str!("messages/not-user-id.html"),
            Self::Incomprehensible => include_str!("messages/incomprehensible.html"),
        }
    }
}

impl AsRef<str> for BotError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<BotError> for &'static str {
    fn from(error: BotError) -> Self {
        error.as_str()
    }
}

impl From<BotError> for String {
    fn from(error: BotError) -> Self {
        error.as_str().to_owned()
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
