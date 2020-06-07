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
  if graph.nlen() <= 10 || graph.elen() <= 10 {
    println!("Read graph:");
    println!("  {:?}", graph);
  } else {
    println!("WARNING: the given graph is too big and cannot be printed.");
  }
  // Run the max clique solver
  let result = solver::solve(&graph, &config.get_solver())?;
  // Check result size
  if result.nlen() <= 10 || result.elen() <= 10 {
    println!("Maximum clique subgraph:");
    println!("  {:?}", result);
  } else {
    println!("WARNING: the resulting graph is too big and cannot be printed.");
  }
  // Check if the result must be saved
  if config.is_save() { io::write(&config.get_filename(), &result)?; }
  // Return Ok
  Ok(())
}
