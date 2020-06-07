extern crate dirs;

use std::error::Error;
use std::fs;
use std::path::Path;
use clap::ArgMatches;
use crate::graph::Graph;

/// Contents of a graph file in a list of pairs format.
type GraphContent = Vec<(usize, usize)>;

/// Options of solver for maximum clique problem.
pub enum Solver {
  Backtracking,
  BranchAndBound,
}

/// Reading configuration.
pub struct Config {
  filename: String,
  solver: Solver,
  save: bool,
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
          "BranchAndBound" => Solver::BranchAndBound,
          _ => return Err("you have entered an invalid solver"),
        };
      } else { return Err("you have entered invalid arguments") }
      // Check if there are still arguments
      let mut save = false;
      if matches.is_present("save") { save = true; }
      // Return the reading configuration
      return Ok(Config{ filename, solver, save })
    }
    Err("you did not enter the filename")
  }

  /// Returns the filename.
  pub fn get_filename(&self) -> &String {
    &self.filename
  }

  /// Returns the solver.
  pub fn get_solver(&self) -> &Solver {
    &self.solver
  }

  /// Returns true if the result must be saved or false otherwise.
  pub fn is_save(&self) -> bool {
    self.save
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

/// Writes a file of a graph.
pub fn write(filename: &String, result: &Graph) -> Result<(), &'static str> {
  // Create directory if it does not exists
  if let Some(dir) = dirs::home_dir() {
    if let Some(dir) = dir.into_os_string().to_str() {
      let target = format!("{}{}", dir, "/max-clique-solutions/");
      if !Path::new(&target).exists() {
        // Check if the folder was successifully created
        if let Err(_) = fs::create_dir(&target) {
          return Err("the result folder could not be created")
        }
      }
      // Fix filename
      let filename = filename.split("/").collect::<Vec<&str>>();
      let filename = filename[filename.len() - 1];
      let filename = format!("{}result_{}", target, filename);
      // Get resulting graph edges
      let mut content = String::new();
      let mut nodes = result.nodes(); nodes.sort();
      for n in nodes { content += format!("{} ", n).as_str(); }
      // Create and write the file
      if let Err(_) = fs::write(&filename, content) {
        return Err("something went wrong during file writting")
      }
      return Ok(())
    }
  }
  Err("the home folder could not be found")
}
