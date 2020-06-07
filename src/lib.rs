//! This file is part of max-clique.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! max-clique is a free software; you can redistribute it and/or modify it
//! under the terms of the MIT License.

pub mod graph;
pub mod io;
pub mod solver;
use std::error::Error;

/// Perform file reading and applies the query.
pub fn run(config: io::Config) -> Result<(), Box<dyn Error>> {
  // Convert file content into a list of pairs (Vec<(usize, usize)>)
  let graph = io::read(&config)?;
  // Run the max clique solver
  solver::solve(&graph, &config);
  // TODO: print result
  todo!("print result");
  // Return Ok
  Ok(())
}
