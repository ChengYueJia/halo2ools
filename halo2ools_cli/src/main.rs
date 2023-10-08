use crate::cli::CliCommand;
use clap::Parser;
use halo2ools::params::{gen_param, gen_param_by_downsize, load_param, save_param};
use std::time::Instant;

mod cli;

#[derive(Parser, Debug)]
#[command(author, version = "0.1.6", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

fn main() {
    let args = Cli::parse();
    let start = Instant::now();
    match args.command {
        CliCommand::GenParam(args) => {
            let param = gen_param(args.degree);
            save_param(&args.param_file, &param);
        }
        CliCommand::DownsizeParam(args) => {
            let src = load_param(&args.src_param);
            let param = gen_param_by_downsize(args.degree, &src);
            save_param(&args.target_param, &param);
        }
    }

    println!("time cost: {}", start.elapsed().as_secs_f64());
}
