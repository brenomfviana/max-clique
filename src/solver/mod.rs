use crate::io::Config;
use crate::graph::Graph;

/// .
pub fn solve(graph: &Graph, config: &Config) -> Result<Graph, &'static str> {
  //
  if graph.is_empty() { return Err("TODO"); }
  //
  Ok(graph.clone())
}
