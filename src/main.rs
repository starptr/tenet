use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
	Ok(())
}
