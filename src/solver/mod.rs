mod backtracking;

use crate::io::Solver;
use crate::graph::Graph;

/// Redirects the graph to the selected solver, run it and return a maximum
/// clique subgraph.
pub fn solve(graph: &Graph, solver: &Solver) -> Result<Graph, &'static str> {
  // Check if the graph is empty
  if graph.is_empty() { return Err("the graph is empty") }
  // If the graph has only one node return the graph
  if graph.nlen() == 1 { return Ok(graph.clone()) }
  // If the graph has two nodes and only one edge return the graph
  if graph.nlen() == 2 && graph.elen() == 1 { return Ok(graph.clone()) }
  // If the graph degree is two return a adjacent pair of nodes
  if graph.degree() == 2 && graph.elen() <= 2 {
    let mut solution = Graph::default();
    let n1 = graph.nodes()[0];
    let n2 = graph.adjlst_of(n1)[0];
    solution.insert_node(n1);
    solution.insert_node(n2);
    solution.insert_edge((n1, n2));
    return Ok(solution)
  }
  // Run solver and return solution
  match solver {
    Solver::Backtracking => Ok(backtracking::solve(&graph)),
    Solver::BranchAndBound => todo!(),
  }
}
