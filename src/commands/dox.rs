//! The dox command.

use super::Command;
use crate::doxee_resolution::{DoxArg, DoxeeSource};
use frakti::{client_cyper::Bot, types::Message};
use futures_util::FutureExt;

/// The dox command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dox {
    pub doxee: Option<String>,
}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    #[allow(clippy::similar_names)]
    fn execute(self, bot: &Bot, msg: Message, _username: &str) -> impl Future<Output = String> {
        let arg = DoxArg::parse(self.doxee.as_deref());
        let source = DoxeeSource::Command { arg, message: msg };
        source.resolve_with(bot).map(|result| {
            match result.expect("command resolution should always reply") {
                Ok(report) => report.to_string(),
                Err(error) => error.to_string(),
            }
        })
    }
}
