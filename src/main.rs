#![allow(unused)]
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(name = "speed", author = "Samuel K.", about = "speed is designed to make the process of compiling and/or running a program streamlined.")]
struct CLIArgs {
  /// The operation to complete.
  op: Option<String>
}

fn main() {
  let args: CLIArgs = CLIArgs::parse();
  args.op.unwrap_or("default".to_string());
}