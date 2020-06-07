//! This file is part of max-clique.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! max-clique is a free software; you can redistribute it and/or modify it
//! under the terms of the MIT License.

#[cfg(test)]
mod tests;

pub mod graph;
pub mod io;
pub mod solver;

use std::error::Error;

/// Perform file reading and applies the query.
pub fn run(config: io::Config) -> Result<(), Box<dyn Error>> {
  // Read the graph from file
  let graph = io::read(&config)?;
  println!("Read graph:");
  println!("  {:?}", graph);
  // Run the max clique solver
  let result = solver::solve(&graph, &config.get_solver())?;
  println!("Maximum clique subgraph:");
  println!("  {:?}", result);
  // TODO: print result
  todo!("print result");
  // Return Ok
  Ok(())
}
