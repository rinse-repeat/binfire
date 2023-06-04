//! This library provides the Runner interface to run binfire

#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(clippy::doc_markdown)]
#![deny(warnings)]

use thiserror::Error;

//mod avail;

#[derive(Error, Debug)]
pub enum RunError {
}

/// binfire runner Options req.
pub trait RunnerOpts {
    fn flag(&self, _: SubCommand) -> bool;
    fn scope_local(&self) -> bool;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SubCommand {
    Available,
    Default,
}

pub struct Runner<C> {
    pub binfire_command: C,
}

impl<C: RunnerOpts> Runner<C> {
    pub fn blocking(binfire_opts: &C) -> Result<(), RunError> {
        dbg!(binfire_opts.flag(SubCommand::Available));
        dbg!(binfire_opts.scope_local());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
