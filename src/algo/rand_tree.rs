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
        let (hex, parent) = self.hex_to_process();
        self.insert_hex(hex.clone(), &parent);
        hex
    }

    fn hex_to_process(&mut self) -> (Hex, Option<Hex>) {
        let rand_index = self.rng.gen_range(0, self.to_process.len());
        self.to_process.swap_remove(rand_index)
    }

    fn assert_forb_prob(forb_prob: f64) {
        match forb_prob {
            fb if 0.0f64 <= fb && fb < 0.9f64 => return,
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

    fn free_forb_neighbors(&mut self, neighbors: NeighborIterator) -> (Vec<Hex>, Vec<Hex>) {
        let mut free = Vec::with_capacity(6);
        let mut forb = Vec::with_capacity(6);
        for n in neighbors {
            let is_busy = self.tree.contains(&n);
            let is_forb = self.forbidden.contains(&n);
            if is_busy || is_forb {
                continue;
            }

            let make_forb = self.rng.gen_bool(self.forb_prob);
            match make_forb {
                true => forb.push(n),
                false => free.push(n),
            }
        }

        (free, forb)
    }

    fn one_forb_to_empty_free(
        &mut self,
        mut free: Vec<Hex>,
        mut forb: Vec<Hex>,
    ) -> (Vec<Hex>, Vec<Hex>) {
        if free.is_empty() {
            if !forb.is_empty() {
                let forb_index = self.rng.gen_range(0, forb.len());
                free.push(forb.swap_remove(forb_index));
            }
        }

        (free, forb)
    }
}

// #[allow(dead_code)]
// pub fn generate(
//     hex_field: &HexField,
//     count: usize,
//     start_point: (f32, f32),
//     forbid_prob: f32,
//     seed: u64,
// ) -> Tree<Hex> {
//     if count == 0 {
//         panic!("can't create empty tree");
//     }

//     let start_hex_center = hex_field.hex_center_by_containing_point(start_point.0, start_point.1);
//     let start_hex = Hex::new(start_hex_center, hex_field.hex_size());
//     let mut rng = StdRng::seed_from_u64(seed);
//     let mut tree: Tree<Hex> = tr(start_hex.clone());
//     let mut to_process: Vec<&mut Node<Hex>> = Vec::with_capacity(count);
//     to_process.push(tree.root_mut());

//     let mut in_tree: HashSet<Hex> = HashSet::with_capacity(count * 2);
//     in_tree.insert(start_hex);

//     let mut forbidden: HashSet<Hex> = HashSet::with_capacity(count * 2);

//     loop {
//         let node = to_process.swap_remove(rng.gen_range(0, to_process.len()));
//         let neighbors = node.data.neighbors();
//         let free = free_neighbors(&neighbors, &in_tree, &forbidden);
//         let (mut conn, forb) = forbid_some_neighbors(free, &mut rng, forbid_prob);

//         let exceed_hex_count = (in_tree.len() + conn.len()) as i64 - count as i64;
//         if exceed_hex_count > 0 {
//             conn = conn[0..conn.len() - exceed_hex_count as usize].to_vec();
//         }

//         let conn_nodes = conn.iter().map(|n| tr(n.clone())).collect::<Vec<_>>();
//         node.extend(conn_nodes);
//         forbidden.extend(forb);
//         in_tree.extend(conn);

//         if in_tree.len() == count {
//             break;
//         }

//         to_process.extend(node.iter_mut());
//     }

//     tree
// }

// fn free_neighbors(neighbors: &[Hex], in_tree: &HashSet<Hex>, forbidden: &HashSet<Hex>) -> Vec<Hex> {
//     neighbors
//         .iter()
//         .filter(|n| !in_tree.contains(n) && !forbidden.contains(n))
//         .cloned()
//         .collect()
// }

// fn forbid_some_neighbors<T: RngCore>(
//     neighbors: Vec<Hex>,
//     rng: &mut T,
//     forbid_prob: f32,
// ) -> (Vec<Hex>, Vec<Hex>) {
//     if neighbors.is_empty() {
//         return (Vec::new(), Vec::new());
//     }

//     let mut keep = Vec::with_capacity(neighbors.len());
//     let mut forb = Vec::with_capacity(neighbors.len());
//     for n in neighbors {
//         let r: f32 = rng.gen();
//         match r {
//             x if x > forbid_prob => keep.push(n),
//             _ => forb.push(n),
//         }
//     }

//     if keep.is_empty() {
//         let forb_index = rng.gen_range(0, forb.len());
//         keep.push(forb.swap_remove(forb_index));
//     }

//     (keep, forb)
// }

// #[cfg(test)]
// mod tests {
//     use super::generate;
//     use crate::hex::Hex;
//     use crate::hex_field::{Config, HexField};
//     use trees::Tree;

//     fn default_hex_field() -> HexField {
//         let width = 19f32;
//         let height = 17f32;
//         let offset_x = width / 2f32;
//         let offset_y = height / 2f32;
//         HexField::new(Config {
//             width,
//             height,
//             offset_x,
//             offset_y,
//         })
//     }

//     fn default_tree(seed: u64) -> Tree<Hex> {
//         let hf = default_hex_field();
//         generate(&hf, 128, (-30f32, -30f32), 0.5f32, seed)
//     }

//     #[test]
//     fn hex_count() {
//         for i in 0..50 {
//             let tree = default_tree(i);
//             assert_eq!(tree.node_count(), 128);
//         }
//     }
// }
