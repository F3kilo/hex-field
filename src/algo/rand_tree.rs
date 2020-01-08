use crate::hex::Hex;
use crate::hex_field::HexField;
use rand::prelude::*;
use std::collections::HashSet;
use trees::{tr, Node, Tree};

#[allow(dead_code)]
pub fn generate(
    hex_field: &HexField,
    count: usize,
    start_point: (f32, f32),
    forbid_prob: f32,
    seed: u64,
) -> Tree<Hex> {
    if count == 0 {
        panic!("can't create empty tree");
    }

    let start_hex_center = hex_field.hex_center_by_containing_point(start_point.0, start_point.1);
    let start_hex = Hex::new(start_hex_center, hex_field.hex_size());
    let mut rng = StdRng::seed_from_u64(seed);
    let mut tree: Tree<Hex> = tr(start_hex.clone());
    let mut to_process: Vec<&mut Node<Hex>> = Vec::with_capacity(count);
    println!("Start: nodes: {};", tree.node_count());

    to_process.push(tree.root_mut());

    let mut in_tree: HashSet<Hex> = HashSet::with_capacity(count * 2);
    in_tree.insert(start_hex);
    let mut forbidden: HashSet<Hex> = HashSet::with_capacity(count * 2);


    loop {
        let node = to_process.swap_remove(rng.gen_range(0, to_process.len()));
        let neighbors = node.data.neighbors();
        let free = free_neighbors(&neighbors, &in_tree, &forbidden);
        let (mut conn, forb) = forbid_some_neighbors(free, &mut rng, forbid_prob);

        println!("Before: In tree: {}; Conn: {};", in_tree.len(), conn.len());
        let exceed_hex_count = (in_tree.len() + conn.len()) as i64 - count as i64;
        if exceed_hex_count > 0 {
            conn = conn[0..conn.len() - exceed_hex_count as usize].to_vec();
        }
        
        println!("After: In tree: {}; Conn: {};", in_tree.len(), conn.len());

        let conn_nodes = conn.iter().map(|n| tr(n.clone())).collect::<Vec<_>>();
        node.extend(conn_nodes);
        forbidden.extend(forb);
        in_tree.extend(conn);

        if in_tree.len() == count {
            break;
        }

        to_process.extend(node.iter_mut());
    }

    tree
}

fn free_neighbors(neighbors: &[Hex], in_tree: &HashSet<Hex>, forbidden: &HashSet<Hex>) -> Vec<Hex> {
    neighbors
        .iter()
        .filter(|n| !in_tree.contains(n) && !forbidden.contains(n))
        .cloned()
        .collect()
}

fn forbid_some_neighbors<T: RngCore>(
    neighbors: Vec<Hex>,
    rng: &mut T,
    forbid_prob: f32,
) -> (Vec<Hex>, Vec<Hex>) {
    if neighbors.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut keep = Vec::with_capacity(neighbors.len());
    let mut forb = Vec::with_capacity(neighbors.len());
    for n in neighbors {
        let r: f32 = rng.gen();
        match r {
            x if x > forbid_prob => keep.push(n),
            _ => forb.push(n),
        }
    }

    if keep.is_empty() {
        let forb_index = rng.gen_range(0, forb.len());
        keep.push(forb.swap_remove(forb_index));
    }

    (keep, forb)
}

#[cfg(test)]
mod tests {
    use super::generate;
    use crate::hex::Hex;
    use crate::hex_field::{Config, HexField};
    use trees::{Tree};

    fn default_hex_field() -> HexField {
        let width = 19f32;
        let height = 17f32;
        let offset_x = width / 2f32;
        let offset_y = height / 2f32;
        HexField::new(Config {
            width,
            height,
            offset_x,
            offset_y,
        })
    }

    fn default_tree() -> Tree<Hex> {
        let hf = default_hex_field();
        generate(&hf, 128, (-30f32, -30f32), 0.5f32, 55u64)
    }

    #[test]
    fn hex_count() {
        let tree = default_tree();
        assert_eq!(tree.node_count(), 128);
    }
}
