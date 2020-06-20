use crate::graph::Graph;

/// Solves the maximum clique problem by using a branch and bound.
pub fn solve(graph: &Graph) -> Graph {
  branch_and_bound(&graph, &graph.nodes_ord_by_degree(), Graph::default())
}

fn branch_and_bound(graph: &Graph, nodes: &[usize], mut clique: Graph)
  -> Graph {
    // Clone current solution
    let mut subgraph = clique.clone();
    // Visit all nodes
    for (i, &n) in nodes.iter().enumerate() {
      // Prune branch if the current `k`-clique subgraph cannot increase
      if clique.degree() >= graph.degree_of(n) { break }
      // Add node
      subgraph.insert_node(n);
      // Add edges
      for c in subgraph.nodes() {
        if graph.adjlst_of(n).contains(&c) {
          subgraph.insert_edge((c, n));
        }
      }
      // Create a search branch and get the branch best solution
      let sol = branch_and_bound(graph, &nodes[i + 1..], subgraph.clone());
      // Check if the branch best solution is better than the current one
      if (sol.is_complete() && clique.is_empty()) ||
        (sol.is_complete() && sol.degree() > clique.degree()) { clique = sol; }
      // Remove added node
      subgraph.remove_node(n);
    }
    clique
}
