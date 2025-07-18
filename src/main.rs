mod chunk;
mod value;
mod vm;

use chunk::{Chunk, Op};
use vm::VM;

use clap::{Parser, Subcommand};
use std::io;
use std::io::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive interpreter
    Repl,

    /// Execute a file
    Run { path: String }
}


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Repl => repl(),
        Commands::Run { path } => {
            println!("run! {}", path)
        }
    }
}

fn repl() {
    // let mut vm = VM::new();

    let mut line_buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin().read_line(&mut line_buffer).expect("Failed to get input!");
        print!("{}", line_buffer);
    }
}
