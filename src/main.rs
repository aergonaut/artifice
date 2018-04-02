extern crate art;
extern crate structopt;

use structopt::StructOpt;

fn main() {
    let opt = art::Art::from_args();
    art::run_command(&opt.command);
    println!("{:?}", opt);
}
