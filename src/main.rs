mod cli;
mod oci;
mod opts;
mod utils;
use clap::Parser;
use cli::Create;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Opts {
    /// Number of times to greet
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

// #[derive(Parser, Debug)]
// enum CommonCmd {
//     Create,
//     Start,
//     Run,
//     Spec,
// }

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
