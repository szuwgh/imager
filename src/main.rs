mod cli;
mod common;
mod container;
mod oci;
mod opts;
mod utils;
use clap::Parser;
use cli::{Create, Start};
use opts::{create, start};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Create(Create),
    Start(Start),
    Run,
    Spec,
}

fn main() {
    let opts = Opts::parse();
    println!("Hello {:?}!", opts.subcmd);
    match opts.subcmd {
        SubCommand::Create(c) => {
            println!("{:?}", c);
            create(c).unwrap();
        }
        SubCommand::Start(s) => {
            start(s).unwrap();
        }
        SubCommand::Run => {}
        SubCommand::Spec => {}
    }
}
