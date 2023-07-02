use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    load:
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
	Ok(())
}
