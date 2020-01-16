use crate::hex::{Hex, NeighborIterator};
use mapped_tree::MappedTree;
use rand::prelude::*;
use std::collections::HashSet;

type ParentHex = Option<Hex>;

pub struct RandHexTree<R: Rng> {
    rng: R,
    forb_prob: f64,
    tree: MappedTree<Hex>,
    to_process: Vec<(Hex, ParentHex)>,
    forbidden: HashSet<Hex>,
}

impl<R: Rng> RandHexTree<R> {
    pub fn new(start_hex: Hex, forb_prob: f64, rng: R) -> Self {
        Self::assert_forb_prob(forb_prob);
        RandHexTree {
            rng,
            forb_prob,
            tree: MappedTree::new(),
            to_process: vec![(start_hex, None)],
            forbidden: HashSet::default(),
        }
    }

    pub fn with_capacity(start_hex: Hex, forb_prob: f64, rng: R, capacity: usize) -> Self {
        Self::assert_forb_prob(forb_prob);
        let mut temp_self = RandHexTree {
            rng,
            forb_prob,
            tree: MappedTree::with_capacity(capacity),
            to_process: Vec::with_capacity(capacity),
            forbidden: HashSet::with_capacity((capacity as f64 * forb_prob) as usize),
        };
        temp_self.to_process.push((start_hex, None));
        temp_self
    }

    pub fn tree(&self) -> &MappedTree<Hex> {
        &self.tree
    }

    pub fn add_hex(&mut self) -> Hex {
        let (hex, parent) = self.rand_free_hex_to_process();
        self.insert_hex(hex.clone(), &parent);
        hex
    }

    pub fn add_hexes(&mut self, count: usize) -> Vec<Hex> {
        let mut hexes = Vec::with_capacity(count);
        for _ in 0..count {
            hexes.push(self.add_hex());
        }
        hexes
    }

    fn rand_free_hex_to_process(&mut self) -> (Hex, ParentHex) {
        let mut rand_hex_with_parent = self.rand_hex_to_process();
        while !self.is_hex_free(&rand_hex_with_parent.0) {
            rand_hex_with_parent = self.rand_hex_to_process();
        }

        rand_hex_with_parent
    }

    fn busy_neigbor(&self, hex: &Hex) -> Hex {
        let busy_neigbor = hex.neighbors().filter(|n| self.is_hex_busy(&n)).next();
        match busy_neigbor {
            Some(n) => n,
            None => panic!("no busy neighbor for forbidden hex"),
        }
    }

    fn rand_hex_to_process(&mut self) -> (Hex, ParentHex) {
        if self.to_process.is_empty() {
            let forbidden_hex = self.forbidden.iter().next().unwrap().clone();
            let busy_neighbor = self.busy_neigbor(&forbidden_hex);
            self.forbidden.remove(&forbidden_hex);
            return (forbidden_hex, Some(busy_neighbor));
        }

        let rand_index = self.rng.gen_range(0, self.to_process.len());
        self.to_process.swap_remove(rand_index)
    }

    fn assert_forb_prob(forb_prob: f64) {
        match forb_prob {
            fb if 0.0f64 <= fb && fb < 0.9f64 => (),
            _ => panic!("Forbidden probability (forb_prob) must be in range: [0, 0.9)"),
        }
    }

    fn insert_hex(&mut self, hex: Hex, parent: &ParentHex) {
        let (free, forb) = self.free_forb_neighbors(hex.neighbors());
        let (free, forb) = self.one_forb_to_empty_free(free, forb);
        let free_with_parents = free.iter().map(|h| (h.clone(), Some(hex.clone())));
        self.forbidden.extend(forb);
        self.to_process.extend(free_with_parents);
        match parent {
            Some(p) => self.tree.insert(hex.clone(), p),
            None => {
                self.tree.reset_root(hex);
            }
        };
    }

    fn is_hex_busy(&self, hex: &Hex) -> bool {
        self.tree.contains(hex)
    }

    fn is_hex_forbidden(&self, hex: &Hex) -> bool {
        self.forbidden.contains(hex)
    }

    fn is_hex_free(&self, hex: &Hex) -> bool {
        !(self.is_hex_busy(hex) || self.is_hex_forbidden(hex))
    }

    fn free_forb_neighbors(&mut self, neighbors: NeighborIterator) -> (Vec<Hex>, Vec<Hex>) {
        let mut free = Vec::with_capacity(6);
        let mut forb = Vec::with_capacity(6);
        for n in neighbors {
            if !self.is_hex_free(&n) {
                continue;
            }

            let make_forb = self.rng.gen_bool(self.forb_prob);
            if make_forb {
                forb.push(n);
            } else {
                free.push(n);
            }
        }
        (free, forb)
    }

    fn one_forb_to_empty_free(
        &mut self,
        mut free: Vec<Hex>,
        mut forb: Vec<Hex>,
    ) -> (Vec<Hex>, Vec<Hex>) {
        if free.is_empty() && !forb.is_empty() {
            let forb_index = self.rng.gen_range(0, forb.len());
            free.push(forb.swap_remove(forb_index));
        }

        (free, forb)
    }
}

#[cfg(test)]
mod tests {
    use super::RandHexTree;
    use crate::hex::Hex;
    use glam::Vec2;
    use rand::prelude::*;

    fn default_tree(seed: u64) -> RandHexTree<StdRng> {
        let h = Hex::new(Vec2::new(20f32, 20f32), Vec2::new(20f32, 20f32));
        RandHexTree::with_capacity(h, 0.5f64, StdRng::seed_from_u64(seed), 200)
    }

    #[test]
    fn hex_count() {
        for i in 50..1000 {
            let mut tree = default_tree(i as u64);
            tree.add_hexes(i);
            assert_eq!(tree.tree().len(), i);
        }
    }
}
