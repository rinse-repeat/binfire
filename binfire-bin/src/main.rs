//! This Binary drives the anyup library

#![forbid(unsafe_code)] 
#![deny(clippy::cargo)]
#![deny(clippy::doc_markdown)]
#![deny(warnings)]

pub mod args;

use binfire_lib::Runner;

fn main() {

    // Argument parsing
    let parsed_args = match args::parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Argument Parsing Error: {}.", e);
            std::process::exit(100);
        }
    };

    // Runner
    match Runner::blocking(&parsed_args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Run Error: {}.", e);
            std::process::exit(10);
        }
    };
}
