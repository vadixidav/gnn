#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};
use space::MetricPoint;

pub struct Gnn<K, V> {
    nodes: Vec<GnnNode<K>>,
    entries: Vec<(K, V)>,
}

pub struct GnnNeighbor<K> {
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
    fn insert(&mut self, key: K, value: V) {
        unimplemented!()
    }
}

impl<K, V> Default for Gnn<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
