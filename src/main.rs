mod cli;
mod oci;
mod opts;
mod utils;
use clap::Parser;
use cli::Create;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Create(Create),
    Start,
    Run,
    Spec,
}

fn main() {
    let opts = Opts::parse();
    println!("Hello {:?}!", opts.subcmd);
    match opts.subcmd {
        SubCommand::Create(create) => {
            println!("{:?}", create);
        }
        SubCommand::Start => {}
        SubCommand::Run => {}
        SubCommand::Spec => {}
    }
}
