//! The dox command.

use super::Command;
use crate::doxee_resolution::{DoxArg, DoxeeSource, resolve};
use frakti::{client_cyper::Bot, types::Message};

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
        let arg = DoxArg::parse(self.doxee.as_deref());
        let source = DoxeeSource::Command { arg, message: msg };
        let result = Box::pin(resolve(bot, source))
            .await
            .expect("command resolution should always reply");
        match result {
            Ok(report) => report.to_string(),
            Err(error) => error.to_string(),
        }
    }
}
