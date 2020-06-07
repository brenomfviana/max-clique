use crate::graph::*;
use crate::io::Solver;
use crate::solver;

#[test]
#[should_panic]
fn backtracking_empty_graph() {
  let graph = Graph::new_empty();
  let _ = solver::solve(&graph, &Solver::Backtracking).unwrap();
}

#[test]
fn backtracking_1_clique() {
  let clique = Graph::new(1);
  let result = solver::solve(&clique, &Solver::Backtracking).unwrap();
  assert_eq!(clique, result);
}

#[test]
fn backtracking_2_clique() {
  let mut clique = Graph::new(2);
  clique.insert_edge((1, 2));
  let result = solver::solve(&clique, &Solver::Backtracking).unwrap();
  assert_eq!(clique, result);
}

#[test]
fn backtracking_3_clique_i() {
  let mut clique = Graph::new(3);
  clique.insert_edge((1, 2));
  clique.insert_edge((1, 3));
  clique.insert_edge((3, 2));
  let result = solver::solve(&clique, &Solver::Backtracking).unwrap();
  let mut clique_nodes = clique.nodes(); clique_nodes.sort();
  let mut result_nodes = result.nodes(); result_nodes.sort();
  assert_eq!(clique_nodes, result_nodes);
  let mut clique_edges = clique.edges(); clique_edges.sort();
  let mut result_edges = result.edges(); result_edges.sort();
  assert_eq!(clique_edges, result_edges);
}

#[test]
fn backtracking_3_clique_ii() {
  let mut graph = Graph::new(5);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  graph.insert_edge((3, 2));
  graph.insert_edge((2, 4));
  graph.insert_edge((3, 4));
  graph.insert_edge((1, 5));
  let mut clique = Graph::new(3);
  clique.insert_edge((1, 2));
  clique.insert_edge((1, 3));
  clique.insert_edge((3, 2));
  let result = solver::solve(&graph, &Solver::Backtracking).unwrap();
  let mut clique_nodes = clique.nodes(); clique_nodes.sort();
  let mut result_nodes = result.nodes(); result_nodes.sort();
  assert_eq!(clique_nodes, result_nodes);
  let mut clique_edges = clique.edges(); clique_edges.sort();
  let mut result_edges = result.edges(); result_edges.sort();
  assert_eq!(clique_edges, result_edges);
}
