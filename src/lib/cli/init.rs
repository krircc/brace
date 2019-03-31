use std::path::Path;

use crate::app::AppConfig;
use crate::util::shell::prelude::*;

pub fn cmd() -> Command {
    Command::new("init")
        .about("Creates a new site in an existing directory")
        .arg(
            Arg::with_name("directory")
                .value_name("DIR")
                .required(true)
                .index(1)
                .help("The target directory"),
        )
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
    let directory = matches.value_of("directory").unwrap();

    match crate::app::init(AppConfig::default(), Path::new(directory)) {
        Ok(()) => {
            shell.info(format!("Created new site at {}", directory))?;
            shell.exit(0);
        }
        Err(err) => {
            shell.error(err)?;
            shell.exit(1);
        }
    }
}
