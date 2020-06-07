use std::collections::HashMap;

/// ```todo!()```.
type AdjMtx = HashMap<usize, Vec<usize>>;

/// This struct represents a graph.
#[derive(Clone, Debug)]
pub struct Graph {
  adjmtx: AdjMtx,
}

impl Graph {
  /// Creates a new graph with disconnected nodes and returns it.
  pub fn new(nodes: usize) -> Graph {
    let mut adjmtx = AdjMtx::new();
    for n in 1..=nodes { adjmtx.insert(n, vec![]); }
    Graph { adjmtx }
  }

  /// Creates a new empty graph and returns it.
  pub fn new_empty() -> Graph {
    Graph { adjmtx: AdjMtx::new() }
  }

  /// ```todo!()```.
  pub fn degre(&self) -> usize {
    todo!();
  }

  /// ```todo!()```.
  pub fn degre_of(&self) -> usize {
    todo!();
  }

  /// ```todo!()```.
  pub fn insert_node(&mut self) {
    todo!();
  }

  /// Inserts an edge in the graph.
  pub fn insert_edge(&mut self, edge: (usize, usize)) {
    if let Some(lst) = self.adjmtx.get_mut(&edge.0) { lst.push(edge.1); }
    else { panic!("The node {} does not belong to this graph.", edge.0); }
    if let Some(lst) = self.adjmtx.get_mut(&edge.1) { lst.push(edge.0); }
    else { panic!("The node {} does not belong to this graph.", edge.1); }
  }

  /// ```todo!()```.
  pub fn get_adjlst_of(&self) -> Vec<usize> {
    todo!();
  }

  /// ```todo!()```.
  pub fn remove(&mut self) {
    todo!();
  }

  /// Returns true if the graph is empty and returns false otherwise.
  pub fn is_empty(&self) -> bool {
    self.adjmtx.is_empty()
  }

  /// Returns the graph lenght.
  pub fn len(&self) -> usize {
    self.adjmtx.len()
  }
}
