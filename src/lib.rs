extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate toml;
extern crate url;

use std::path::PathBuf;

use failure::Error;

mod commands;
mod config;
mod jira;

/// The global struct representing the whole executable.
///
/// Flags listed here are global to the executable, while flags specific to a subcommand are
/// contained in the `Command` struct.
#[derive(Debug, StructOpt)]
#[structopt(name = "art")]
pub struct Art {
    /// Verbose output.
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbose: u64,
    /// Config file. Defaults to $HOME/.artifice.toml
    #[structopt(long = "config", short = "c", parse(from_os_str))]
    config: Option<PathBuf>,
    #[structopt(subcommand)]
    command: Command,
}

/// The core enum describing the different available subcommands.
///
/// Each variant of `Command` represents a different subcommand of the executable. The variants
/// contain the flags and arguments passed to the subcommand specifically.
#[derive(Debug, StructOpt)]
pub enum Command {
    /// Start work on a new ticket. Specify the JIRA issue key directly, or choose from a list of
    /// your open tickets in the current sprint.
    #[structopt(name = "start")]
    Start {
        /// JIRA ticket to start
        #[structopt(name = "TICKET")]
        ticket: Option<String>,
    },
}

pub fn run_command(opts: &Art) -> Result<(), Error> {
    let config = config::Config::open(&opts.config)?;
    match opts.command {
        Command::Start { ref ticket } => commands::start_command(ticket, &config),
    }
}

impl Art {
    pub fn verbosity(&self) -> log::LevelFilter {
        match self.verbose {
            x if x == 0 => log::LevelFilter::Error,
            x if x == 1 => log::LevelFilter::Warn,
            _ => log::LevelFilter::Info,
        }
    }
}
