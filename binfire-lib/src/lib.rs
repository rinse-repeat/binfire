//! This library provides the Runner interface to run binfire

#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(clippy::doc_markdown)]
//#![deny(warnings)]

use thiserror::Error;

//mod avail;

mod add;
pub use add::CommandAddOpts;

#[derive(Error, Debug)]
pub enum RunError {
    Add(String),
}

use core::fmt;
use core::fmt::Display;

impl Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::Add(s) => write!(f, "ERROR: Add: {}", s),
        }
    }
}

/// binfire runner Options req.
pub trait RunnerOpts {
    fn flag(&self, _: SubCommand) -> bool;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SubCommand {
    Available,
    Default,
    Add,
}

pub struct Runner<C> {
    pub command: C,
}

impl<C: RunnerOpts + CommandAddOpts> Runner<C> {
    pub async fn run(binfire_opts: &C) -> Result<(), RunError> {
        dbg!(binfire_opts.flag(SubCommand::Available));
        dbg!(binfire_opts.flag(SubCommand::Add));

        if binfire_opts.flag(SubCommand::Add) {
            let _ = crate::add::AddRunner::run(binfire_opts)
                .await
                .map_err(|e| RunError::Add(e.to_string()))?;
        }
        Ok(())
    }
}

/* whatever
#[cfg(test)]
mod tests {
    use super::*;

}
 */
