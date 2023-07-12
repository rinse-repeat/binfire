use super::*;

use core::fmt;
use core::fmt::Display;
use regex::Regex;
use serde::Deserialize;
use url::Url;

#[derive(Error, Debug)]
pub enum AddRunError {
    Fetch(String),
    ParseResponse(String),
    ParseSpec(String),
}

impl Display for AddRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddRunError::Fetch(s) => write!(f, "Cannot Add - fetch: {}", s),
            AddRunError::ParseResponse(s) => write!(f, "Cannot parse response: {}", s),
            AddRunError::ParseSpec(s) => write!(f, "Invalid SPEC: {}", s),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AddRunner<O> {
    opts: O,
}

/// binfire add Options
pub trait CommandAddOpts {
    fn spec(&self) -> &str;
}

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Channel {
    current: String,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    channel: HashMap<String, Channel>,
    versions: HashMap<String, Vec<String>>,
}

macro_rules! matcher_regex {
    ($re:expr $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

impl<O: CommandAddOpts> AddRunner<O> {
    pub async fn run(binfire_add_opts: &O) -> Result<(), AddRunError> {
        dbg!(binfire_add_opts.spec());

        let client = reqwest::Client::new();

        let scheme = Url::parse(binfire_add_opts.spec())
            .map_err(|e| AddRunError::ParseSpec(e.to_string()))?;

        dbg!(scheme);

        return Ok(());

        let set_url: String =
            "https://cdn.jsdelivr.net/gh/rinse-repeat/binfire/binfire.toml".into();

        let response = client
            .get(set_url)
            .send()
            .await
            .map_err(|e| AddRunError::Fetch(e.to_string()))?;

        let data = response
            .text()
            .await
            .map_err(|e| AddRunError::Fetch(e.to_string()))?;

        let manifest: Manifest =
            toml::from_str(data.as_str()).map_err(|e| AddRunError::ParseResponse(e.to_string()))?;

        dbg!(manifest);

        Ok(())
    }
}
