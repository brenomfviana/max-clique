//! This file is part of max-clique.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! max-clique is a free software; you can redistribute it and/or modify it
//! under the terms of the MIT License.

use std::env;
use std::process;
use max_clique::io::Config;

fn main() {
  // Get arguments
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  // Perform the search
  if let Err(e) = max_clique::run(config) {
    eprintln!("Application error: {}.", e);
    process::exit(1);
  }
}
