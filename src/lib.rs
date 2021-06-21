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
    K: MetricPoint,
{
    fn insert(&mut self, key: K, value: V) -> usize {
        // First we perform a greedy search to find the greedy destination.
        if let Some(greedy_nearest) = self.greedy_search(&key) {
            // Once we have the nearest neighbor from the greedy search,
            // we need to find the real nearest neighbor.
            unimplemented!()
        } else {
            // In this case, there is nothing in the GNN, so add the item and return.
            self.nodes.push(GnnNode { neighbors: vec![] });
            self.entries.push((key, value));
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
}

impl<K, V> Default for Gnn<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
