use std::cmp;
use std::collections::HashMap;

/// ```todo!()```.
type AdjMtx = HashMap<usize, Vec<usize>>;

/// This struct represents a graph.
#[derive(Clone, Debug)]
pub struct Graph {
  adjmtx: AdjMtx,
  degree: usize,
}

impl Graph {
  /// Creates a new graph with disconnected nodes and returns it.
  pub fn new(nodes: usize) -> Graph {
    assert!(nodes != 0, "The number of nodes cannot be zero.");
    let mut adjmtx = AdjMtx::new();
    for n in 1..=nodes { adjmtx.insert(n, vec![]); }
    Graph { adjmtx, degree: 0 }
  }

  /// Creates a new empty graph and returns it.
  pub fn new_empty() -> Graph {
    Graph { adjmtx: AdjMtx::new(), degree: 0 }
  }

  /// Returns the graph degree.
  pub fn degree(&self) -> usize {
    self.degree
  }

  /// Returns the degree of a node.
  pub fn degree_of(&self, n: usize) -> usize {
    assert!(self.contains_node(n),
      "The given node does not belong to the graph");
    self.adjmtx[&n].len()
  }

  /// Returns the adjacency list of a node.
  pub fn get_adjlst_of(&self, n: usize) -> &Vec<usize> {
    &self.adjmtx[&n]
  }

  /// Inserts a new node in the graph.
  pub fn insert_node(&mut self, n: usize) {
    assert!(!self.contains_node(n),
      "The given node already belongs to the graph");
    self.adjmtx.insert(n, vec![]);
  }

  /// Inserts an edge in the graph.
  pub fn insert_edge(&mut self, (a, b): (usize, usize)) {
    if let Some(lst) = self.adjmtx.get_mut(&a) {
      lst.push(b);
      self.degree = cmp::max(self.degree, lst.len());
    }
    else { panic!("The node {} does not belong to this graph.", a); }
    if let Some(lst) = self.adjmtx.get_mut(&b) {
      lst.push(a);
      self.degree = cmp::max(self.degree, lst.len());
    }
    else { panic!("The node {} does not belong to this graph.", b); }
  }

  /// Removes a node from the graph.
  pub fn remove_node(&mut self, n: usize) {
    assert!(self.contains_node(n),
      "This graph does not contains the given node.");
    self.adjmtx.remove(&n);
    for (_, v) in self.adjmtx.iter_mut() {
      if let Some(index) = v.iter().position(|x| *x == n) { v.remove(index); }
    }
  }

  /// Removes an edge from the graph.
  pub fn remove_edge(&mut self, (a, b): (usize, usize)) {
    assert!(self.contains_node(a) && self.contains_node(b),
      "This graph does not contains at least one of the given nodes.");
    assert!(self.adjmtx[&a].contains(&b) && self.adjmtx[&b].contains(&a),
      "The first node is not adjacent to the second one");
    if let Some(v) = self.adjmtx.get_mut(&a) {
      if let Some(index) = v.iter().position(|x| *x == b) { v.remove(index); }
    }
    if let Some(v) = self.adjmtx.get_mut(&b) {
      if let Some(index) = v.iter().position(|x| *x == a) { v.remove(index); }
    }
  }

  /// Returns true if the graph contains the node and false otherwise.
  pub fn contains_node(&self, n: usize) -> bool {
    self.adjmtx.contains_key(&n)
  }

  /// Returns true if the graph contains the edge and false otherwise.
  pub fn contains_edge(&self, e: (usize, usize)) -> bool {
    if !self.adjmtx.contains_key(&e.0) { return false; }
    if !self.adjmtx.contains_key(&e.1) { return false; }
    if !self.adjmtx[&e.0].contains(&e.1) { return false; }
    if !self.adjmtx[&e.1].contains(&e.0) { return false; }
    true
  }

  /// ```todo!()```.
  pub fn is_complete(&self) -> bool {
    todo!();
  }

  /// Returns true if the graph is empty and false otherwise.
  pub fn is_empty(&self) -> bool {
    if self.adjmtx.is_empty() { assert!(self.degree == 0); }
    self.adjmtx.is_empty()
  }

  /// Returns the number of nodes of the graph.
  pub fn nlen(&self) -> usize {
    self.adjmtx.len()
  }

  /// Returns the number of edges of the graph.
  pub fn elen(&self) -> usize {
    let mut sum = 0;
    for (_, adjlst) in &self.adjmtx { sum += adjlst.iter().len(); }
    sum / 2
  }
}
