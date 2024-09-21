mod config;
mod render;
mod run;

use config::Config;
use run::run;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect::<Vec<String>>();
    let config = Config::from(args);

    run(config);
}
