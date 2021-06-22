#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};
use space::MetricPoint;

pub struct Gnn<K, V> {
    nodes: Vec<GnnNode<K>>,
    entries: Vec<(K, V)>,
}

struct GnnNeighbor<K> {
    key: K,
    node: usize,
}

pub struct GnnNode<K> {
    neighbors: Vec<GnnNeighbor<K>>,
}

impl<K, V> Gnn<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            entries: vec![],
        }
    }
}

impl<K, V> Gnn<K, V>
where
    K: MetricPoint + Clone,
{
    fn insert(&mut self, insert_key: K, insert_value: V) -> usize {
        // First we perform a greedy search to find the greedy destination.
        if let Some(greedy_nn) = self.greedy_search(&insert_key) {
            // Once we have the nearest neighbor from the greedy search,
            // we need to find the real nearest neighbor.
            let real_nn = self
                .entries
                .iter()
                .enumerate()
                .min_by_key(|(_, (key, value))| insert_key.distance(key))
                .unwrap()
                .0;

            // We need to point the greedy nearest neighbor, which is along the greedy search path,
            // to the real nearest neighbor. This allows a greedy search path to the target which also intersects
            // with the nearest neighbor.
            self.add_neighbor(greedy_nn, real_nn);

            // Insert the node itself.
            let insert_node = self.entries.len();
            self.nodes.push(GnnNode { neighbors: vec![] });
            self.entries.push((insert_key, insert_value));

            // Connect the node to its nearest neighbor.
            self.add_neighbor(real_nn, insert_node);

            insert_node
        } else {
            // In this case, there is nothing in the GNN, so add the item and return.
            self.nodes.push(GnnNode { neighbors: vec![] });
            self.entries.push((insert_key, insert_value));
            0
        }
    }

    fn greedy_search(&self, query_key: &K) -> Option<usize> {
        if self.entries.is_empty() {
            None
        } else {
            let mut to_search = vec![0];
            let mut best_distance = query_key.distance(&self.entries[0].0);
            let mut best_index = 0;

            while let Some(search_node) = to_search.pop() {
                for GnnNeighbor { key, node } in &self.nodes[search_node].neighbors {
                    let distance = query_key.distance(key);
                    if distance < best_distance {
                        to_search.push(*node);
                        best_index = *node;
                        best_distance = distance;
                    }
                }
            }

            Some(best_index)
        }
    }

    /// Adds an edge to the DAG. This will not add the edge if it already exists.
    pub fn add_neighbor(&mut self, source: usize, target: usize) {
        // First, check if the neighbor already exists.
        if self.nodes[source]
            .neighbors
            .iter()
            .any(|neighbor| neighbor.node == target)
        {
            // If it does, don't add it.
            return;
        }
        let neighbor = GnnNeighbor {
            key: self.entries[target].0.clone(),
            node: target,
        };

        self.nodes[source].neighbors.push(neighbor);
    }
}

impl<K, V> Default for Gnn<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
