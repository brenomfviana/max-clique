//! This file is part of max-clique.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! max-clique is a free software; you can redistribute it and/or modify it
//! under the terms of the MIT License.

use std::error::Error;

pub mod graph;
pub mod io;

use io::Config;

/// Perform file reading and applies the query.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // Convert file content into a list of pairs (Vec<(usize, usize)>)
  let graph = io::read(config)?;
  println!("{:?}", graph);
  // Run the max clique solver
  todo!("SOLVER");
  // Return Ok
  Ok(())
}
