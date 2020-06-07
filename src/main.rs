//! This file is part of max-clique.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! max-clique is a free software; you can redistribute it and/or modify it
//! under the terms of the MIT License.

#[macro_use]
extern crate clap;

use clap::App;
use std::process;
use max_clique::io::Config;

fn main() {
  // Read cli configuration from `cli.yml`
  let cli = load_yaml!("cli.yml");
  // Load cli from `cliconf.yml`
  let matches = App::from_yaml(cli).get_matches();
  // Get configuration
  let config = Config::new(matches).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}.", err);
    process::exit(1);
  });
  // Run the maximum clique solver
  if let Err(e) = max_clique::run(config) {
    eprintln!("Application error: {}.", e);
    process::exit(1);
  }
}
