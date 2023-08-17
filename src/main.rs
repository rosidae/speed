#![allow(unused)]
use clap::Parser;
use std::fs;
use yaml_rust::{YamlLoader, YamlEmitter};

#[derive(Parser)]
#[command(name = "speed", author = "Samuel K.", about = "speed is designed to make the process of compiling and/or running a program streamlined.")]
struct CLIArgs {
  /// The operation to complete.
  #[arg(default_value = "main")]
  op: Option<String>,
  /// The YAML configuration file to use
  #[arg(short = 'c', long = "config", alias = "file", default_value = "speed.yml")]
  config: Option<String>,
}

fn main() {
  let args: CLIArgs = CLIArgs::parse();
  let operation_name: String = args.op.unwrap_or("main".to_string());
  let config_file: String = args.config.unwrap_or("speed.yml".to_string());
  let config: String = fs::read_to_string(config_file).expect("Something went wrong reading the file");
  let docs = YamlLoader::load_from_str(&config).unwrap();

}