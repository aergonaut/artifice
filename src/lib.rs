extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate toml;

use failure::Error;

mod config;

#[derive(Debug, StructOpt)]
#[structopt(name = "art")]
pub struct Art {
    /// Verbose output.
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbose: u64,
    #[structopt(subcommand)]
    pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Start work on a new ticket. Specify the JIRA issue key directly, or choose from a list of
    /// your open tickets in the current sprint.
    #[structopt(name = "start")]
    Start {
        /// JIRA ticket to start
        #[structopt(name = "TICKET")]
        ticket: Option<String>
    }
}

pub fn run_command(cmd: &Command) -> Result<(), Error> {
    let config = config::Config::get()?;
    match *cmd {
        Command::Start { ref ticket } => start_command(ticket, &config)
    }
}

fn start_command(ticket: &Option<String>, config: &config::Config) -> Result<(), Error> {

}
