extern crate art;
extern crate fern;
#[macro_use]
extern crate log;
extern crate structopt;

use fern::colors::{Color, ColoredLevelConfig};
use structopt::StructOpt;

fn main() {
    let opt = art::Art::from_args();

    // Configure logger
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(opt.verbosity())
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    info!("Starting art");
    match art::run_command(&opt) {
        Ok(_) => {}
        Err(error) => error!("{}", error),
    }
    info!("Art exited");
}
