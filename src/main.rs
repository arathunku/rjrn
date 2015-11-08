extern crate rjrn;
extern crate rustc_serialize;

use rjrn::{simple_logger};

mod cli;
mod cli_args;

fn main() {
    let args: cli_args::Args = cli_args::get();
    simple_logger::init(args.flag_verbose)
        .ok()
        .expect("Something went wrong with the logger");

    match cli::process_args(&args) {
        Ok(_) => (),
        Err(why) => println!("There was an error: {}", why)
    }
}
