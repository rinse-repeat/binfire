use binfire_lib::RunnerOpts;
use binfire_lib::SubCommand;
use regex::Regex;

const HELP: &str = "\
binfire

USAGE:
  binfire [OPTIONS] COMMAND [ARGS]

COMMAND:
  add [SPEC]               Add a manifest
  nuke [SPEC]              Remove a manifest
  list                     Lists manifests
  update                   Update all manifests
  app [APP] [APP_COMMAND]  Manage APP

APP_COMMAND:
  avail                    List available flavors
  install [FLAVOR]         Install [flavor]
  def [FLAVOR]             Switch default to [flavor]
  update                   Update APP

SPEC:
  github:user/repo         Add from GitHub
  https://..               Add by URL

FLAVOR:
  nightly                  Nightly built
  stable                   Stable
  1.0.0                    Version 1.0.0
  PR/1                     Pull Request 1
  PR/1/commit/avcdef       PR by commit
  Commit/main/abcdef       By commit
  ..                       App defined

";

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedOpts {
    sub_command: SubCommand,
}

impl RunnerOpts for ParsedOpts {
    fn flag(&self, cmd: SubCommand) -> bool {
        self.sub_command == cmd
    }
}

fn help_and_error(e: Option<&str>) -> ! {
    let mut code = 0;
    if let Some(e) = e {
        eprintln!("Error: {}", e);
        code = 1;
    };
    print!("{}", HELP);

    std::process::exit(code)
}

#[derive(Debug)]
pub enum ArgsError {}

pub fn parse() -> Result<ParsedOpts, ArgsError> {
    let mut args = std::env::args();

    let re_subcmds = vec![
        (
            SubCommand::Available,
            Regex::new(r"^avail").expect("BUG: Subcommand avail regex faulty"),
        ),
        (
            SubCommand::Add,
            Regex::new(r"^add").expect("BUG: Subcommand add regex faulty"),
        ),
        (
            SubCommand::Default,
            Regex::new(r"^def").expect("BUG: Subcommand def regex faulty"),
        ),
    ];

    // Is there a subcommand at all?
    let find_subcmd = match args.next() {
        Some(subcmd) => re_subcmds.iter().find(|&x| x.1.is_match(&subcmd)),
        None => help_and_error(None),
    };

    let found_subcmd = match find_subcmd {
        Some(v) => v.0.to_owned(),
        None => help_and_error(Some("Unknown subcomamnd")),
    };

    // help_and_error(Some(&format!("Unknown arguments: {:?}.", remaining)));

    let binfire_args = ParsedOpts {
        sub_command: found_subcmd,
    };

    Ok(binfire_args)
}
