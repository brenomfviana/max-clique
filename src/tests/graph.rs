use crate::graph::*;

#[test]
fn default_graph() {
  let graph = Graph::default();
  assert_eq!(graph.is_empty(), true);
  assert_eq!(graph.degree(), 0);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![]);
}

#[test]
fn new_graph() {
  let graph = Graph::new(2);
  assert_eq!(graph.nlen(), 2);
  assert_eq!(graph.degree(), 0);
  assert_eq!(graph.is_empty(), false);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2]);
}

#[test]
#[should_panic]
fn invalid_new_graph() {
  let _ = Graph::new(0);
}

#[test]
fn adding_edges_i() {
  let mut graph = Graph::new(2);
  graph.insert_edge((1, 2));
  assert_eq!(graph.nlen(), 2);
  assert_eq!(graph.elen(), 1);
  assert_eq!(graph.degree(), 1);
  assert_eq!(graph.degree_of(1), 1);
  assert_eq!(graph.degree_of(2), 1);
  assert_eq!(graph.contains_node(1), true);
  assert_eq!(graph.contains_node(2), true);
  assert_eq!(graph.contains_edge((1, 2)), true);
  assert_eq!(graph.is_empty(), false);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2]);
  let mut edges = graph.edges(); edges.sort();
  assert_eq!(edges, vec![(1, 2)]);
}

#[test]
fn adding_edges_ii() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  graph.insert_edge((2, 3));
  assert_eq!(graph.nlen(), 3);
  assert_eq!(graph.elen(), 3);
  assert_eq!(graph.degree(), 2);
  assert_eq!(graph.degree_of(1), 2);
  assert_eq!(graph.degree_of(2), 2);
  assert_eq!(graph.degree_of(3), 2);
  assert_eq!(graph.contains_node(1), true);
  assert_eq!(graph.contains_node(2), true);
  assert_eq!(graph.contains_node(3), true);
  assert_eq!(graph.contains_edge((1, 2)), true);
  assert_eq!(graph.contains_edge((1, 3)), true);
  assert_eq!(graph.contains_edge((2, 3)), true);
  assert_eq!(graph.is_empty(), false);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3]);
  let mut edges = graph.edges(); edges.sort();
  assert_eq!(edges, vec![(1, 2), (1, 3), (2, 3)]);
}

#[test]
fn adding_edges_iii() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(graph.nlen(), 3);
  assert_eq!(graph.elen(), 2);
  assert_eq!(graph.degree(), 2);
  assert_eq!(graph.degree_of(1), 2);
  assert_eq!(graph.degree_of(2), 1);
  assert_eq!(graph.degree_of(3), 1);
  assert_eq!(graph.contains_node(1), true);
  assert_eq!(graph.contains_node(2), true);
  assert_eq!(graph.contains_node(3), true);
  assert_eq!(graph.contains_edge((1, 2)), true);
  assert_eq!(graph.contains_edge((1, 3)), true);
  assert_eq!(graph.is_empty(), false);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3]);
  let mut edges = graph.edges(); edges.sort();
  assert_eq!(edges, vec![(1, 2), (1, 3)]);
}

#[test]
#[should_panic]
fn invalid_adding_edges() {
  let mut graph = Graph::new(2);
  graph.insert_edge((0, 1));
}

#[test]
fn do_not_contains_node() {
  let mut graph = Graph::new(2);
  graph.insert_edge((1, 2));
  assert_eq!(graph.contains_node(0), false);
}

#[test]
fn do_not_contains_edge() {
  let mut graph = Graph::new(2);
  graph.insert_edge((1, 2));
  assert_eq!(graph.contains_edge((0, 1)), false);
}

#[test]
fn adjacency_list() {
  let mut graph = Graph::new(4);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
  assert_eq!(*graph.adjlst_of(2), vec![1]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  assert_eq!(*graph.adjlst_of(4), vec![]);
}

#[test]
#[should_panic]
fn invalid_adjacency_list() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((2, 3));
  assert_eq!(*graph.adjlst_of(1), vec![]);
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
}

#[test]
fn insert_node_i() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
  assert_eq!(*graph.adjlst_of(2), vec![1]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3]);
  graph.insert_node(4);
  assert_eq!(*graph.adjlst_of(4), vec![]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3, 4]);
}

#[test]
fn insert_node_ii() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
  assert_eq!(*graph.adjlst_of(2), vec![1]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3]);
  graph.insert_node(4);
  graph.insert_edge((4, 3));
  assert_eq!(*graph.adjlst_of(3), vec![1, 4]);
  assert_eq!(*graph.adjlst_of(4), vec![3]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn invalid_insert_node() {
  let mut graph = Graph::new(3);
  graph.insert_node(1);
}

#[test]
fn remove_node_i() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
  assert_eq!(*graph.adjlst_of(2), vec![1]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3]);
  graph.remove_node(2);
  assert_eq!(*graph.adjlst_of(1), vec![3]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 3]);
}

#[test]
fn remove_node_ii() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
  assert_eq!(*graph.adjlst_of(2), vec![1]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![1, 2, 3]);
  graph.remove_node(1);
  assert_eq!(*graph.adjlst_of(2), vec![]);
  assert_eq!(*graph.adjlst_of(3), vec![]);
  let mut nodes = graph.nodes(); nodes.sort();
  assert_eq!(nodes, vec![2, 3]);
}

#[test]
#[should_panic]
fn invalid_remove_node() {
  let mut graph = Graph::new(3);
  graph.remove_node(4);
}

#[test]
fn remove_edge() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  assert_eq!(*graph.adjlst_of(1), vec![2, 3]);
  assert_eq!(*graph.adjlst_of(2), vec![1]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut edges = graph.edges(); edges.sort();
  assert_eq!(edges, vec![(1, 2), (1, 3)]);
  graph.remove_edge((1, 2));
  assert_eq!(*graph.adjlst_of(1), vec![3]);
  assert_eq!(*graph.adjlst_of(2), vec![]);
  assert_eq!(*graph.adjlst_of(3), vec![1]);
  let mut edges = graph.edges(); edges.sort();
  assert_eq!(edges, vec![(1, 3)]);
}

#[test]
#[should_panic]
fn invalid_remove_edge() {
  let mut graph = Graph::new(3);
  graph.remove_edge((1, 2));
}

#[test]
fn is_complete_i() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  graph.insert_edge((2, 3));
  assert!(graph.is_complete());
}

#[test]
fn is_complete_ii() {
  let mut graph = Graph::new(4);
  graph.insert_edge((1, 2));
  graph.insert_edge((1, 3));
  graph.insert_edge((1, 4));
  graph.insert_edge((2, 3));
  graph.insert_edge((2, 4));
  graph.insert_edge((3, 4));
  assert!(graph.is_complete());
}

#[test]
#[should_panic]
fn is_not_complete_i() {
  let mut graph = Graph::new(3);
  graph.insert_edge((1, 2));
  graph.insert_edge((2, 3));
  assert!(graph.is_complete());
}

#[test]
#[should_panic]
fn is_not_complete_ii() {
  let mut graph = Graph::new(4);
  graph.insert_edge((1, 2));
  graph.insert_edge((2, 3));
  assert!(graph.is_complete());
}
