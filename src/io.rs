use std::error::Error;
use std::fs;

use crate::graph::Graph;

/// Contents of a graph file in a list of pairs format.
type GraphContent = Vec<(usize, usize)>;

/// Reading configuration.
pub struct Config {
  filename: String,
}

impl Config {
  /// Validates the arguments and returns the reading configuration.
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    // Skip program call
    args.next();
    // Get query filename from arguments
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("you did not enter the filename"),
    };
    // Check if there are still arguments
    if let Some(_) = args.next() {
      println!("  WARNING: You have entered more arguments than needed.");
    }
    // Return the reading configuration
    Ok(Config{ filename })
  }

  /// Returns the filename.
  pub fn get_filename(&self) -> &String {
    &self.filename
  }
}

/// Reads a graph file and returns the respective graph.
pub fn read(config: Config) -> Result<Graph, Box<dyn Error>> {
  // Read the file
  let content = fs::read_to_string(config.get_filename())?;
  // Split content by '\n'
  let lines: Vec<&str> = content.split("\n").collect();
  // Create an empty list of pairs (graph content)
  let mut gc = GraphContent::new();
  // Read the pairs
  for line in lines {
    // Get list of chars of the line
    let chrs = line.split(" ").collect::<Vec<&str>>();
    // Ignore comments
    if chrs[0] == "c" { continue; }
    // Check if the line has less than 2 characters
    if chrs.len() <= 2 { panic!("Invalid file!"); }
    // Convert into pair
    let pair: Vec<usize> = (chrs[(chrs.len() - 2)..]).to_vec()
      .iter().map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the pair has exactly 2 characters
    if pair.len() != 2 { panic!("Invalid file!"); }
    // Add the number of nodes and edges
    gc.push((pair[0], pair[1]));
  }
  // Create the graph
  let mut graph = Graph::new(gc[0].0);
  for e in 1..=gc[0].1 { graph.insert_edge(gc[e]); }
  // Return the graph
  Ok(graph)
}
