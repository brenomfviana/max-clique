use std::error::Error;
use std::fs;
use clap::ArgMatches;
use crate::graph::Graph;

/// Contents of a graph file in a list of pairs format.
type GraphContent = Vec<(usize, usize)>;

/// Options of solver for maximum clique problem.
pub enum Solver {
  Backtracking,
  BranchAndBounds,
}

/// Reading configuration.
pub struct Config {
  filename: String,
  solver: Solver,
}

impl Config {
  /// Validates the arguments and returns the reading configuration.
  pub fn new(matches: ArgMatches) -> Result<Config, &'static str> {
    // Get query filename from arguments
    if let Some(filename) = matches.value_of("filename") {
      // Convert filename to string
      let filename = filename.to_string();
      // Check if there are still arguments
      let solver;
      if let Some(s) = matches.value_of("solver") {
        solver = match s {
          "Backtracking" => Solver::Backtracking,
          "BranchAndBounds" => Solver::BranchAndBounds,
          _ => return Err("you have entered an invalid solver"),
        };
      } else { return Err("you have entered invalid arguments") }
      // Return the reading configuration
      Ok(Config{ filename, solver })
    }
    else { Err("you did not enter the filename") }
  }

  /// Returns the filename.
  pub fn get_filename(&self) -> &String {
    &self.filename
  }

  /// Returns the solver.
  pub fn get_solver(&self) -> &Solver {
    &self.solver
  }
}

/// Reads a graph file and returns the respective graph.
pub fn read(config: &Config) -> Result<Graph, Box<dyn Error>> {
  // Read the file
  let content = fs::read_to_string(config.get_filename())?;
  // Split content by '\n'
  let lines: Vec<&str> = content.split("\n").collect();
  // Create an empty list of pairs (graph content)
  let mut gc = GraphContent::new();
  // Read the pairs
  for (i, line) in lines.iter().enumerate() {
    // Get list of chars of the line
    let chrs = line.split(" ").collect::<Vec<&str>>();
    // Ignore comments
    if line.len() == 0 || chrs[0] == "c" { continue; }
    // Check if the line has less than 2 characters
    if chrs.len() <= 2 { panic!("Invalid pair at line {}!", i + 1); }
    // Convert into pair
    let pair: Vec<usize> = (chrs[(chrs.len() - 2)..]).to_vec()
      .iter().map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the pair has exactly 2 characters
    if pair.len() != 2 { panic!("The line {} has not a valid pair!", i + 1); }
    // Add the number of nodes and edges
    gc.push((pair[0], pair[1]));
  }
  // Check if the graph has the right number of edges
  if gc[0].1 != gc.len() - 1 { panic!("Invalid number of edges!"); }
  // Create the graph
  let mut graph = Graph::new(gc[0].0);
  for e in 1..=gc[0].1 { graph.insert_edge(gc[e]); }
  // Return the graph
  Ok(graph)
}
