//! Graph scheduling for processing order

use crate::{AudioGraph, NodeId};
use std::collections::{HashMap, HashSet, VecDeque};

/// Computes the processing order for an audio graph using topological sort
pub struct GraphScheduler;

impl GraphScheduler {
    /// Compute the processing order for the graph
    ///
    /// Returns nodes in the order they should be processed
    pub fn compute_order(_graph: &AudioGraph, connections: &[(NodeId, NodeId)]) -> Vec<NodeId> {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut adj_list: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut all_nodes: HashSet<NodeId> = HashSet::new();

        // Build adjacency list and in-degree count
        for (source, target) in connections {
            all_nodes.insert(*source);
            all_nodes.insert(*target);
            adj_list.entry(*source).or_default().push(*target);
            *in_degree.entry(*target).or_default() += 1;
        }

        // Initialize in-degree for nodes with no incoming edges
        for node in &all_nodes {
            in_degree.entry(*node).or_insert(0);
        }

        // Kahn's algorithm for topological sort
        let mut queue: VecDeque<NodeId> = VecDeque::new();
        let mut result = Vec::new();

        // Start with nodes that have no incoming edges
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(*node);
            }
        }

        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(neighbors) = adj_list.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != all_nodes.len() {
            // Graph has a cycle - return partial result
            // In a real implementation, this should be an error
            tracing::warn!("Audio graph contains a cycle!");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_chain() {
        let connections = vec![
            (NodeId(0), NodeId(1)),
            (NodeId(1), NodeId(2)),
        ];

        let order = GraphScheduler::compute_order(&AudioGraph::new(), &connections);
        assert_eq!(order, vec![NodeId(0), NodeId(1), NodeId(2)]);
    }

    #[test]
    fn test_parallel_paths() {
        let connections = vec![
            (NodeId(0), NodeId(2)),
            (NodeId(1), NodeId(2)),
        ];

        let order = GraphScheduler::compute_order(&AudioGraph::new(), &connections);
        assert!(order.len() == 3);
        // Node 2 should be last
        assert_eq!(order[2], NodeId(2));
    }
}
