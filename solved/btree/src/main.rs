use core::num;

use rand::RngExt;

#[derive(Debug)]
struct BTree {
    node: i32,
    left: Option<Box<BTree>>,
    right: Option<Box<BTree>>
}

fn print_tree(tree: &Option<Box<BTree>>, depth: usize) {
    if let Some(node) = tree {
        print_tree(&node.right, depth + 1);
        for _ in 0..depth {
            print!("    ");
        }
        println!("{}", node.node);
        print_tree(&node.left, depth + 1);
    }
}

fn build_tree(sorted: Vec<i32>) -> Option<Box<BTree>> {
    if sorted.len() == 0 {
        return None;
    }
    if sorted.len() == 1 {
        let value = sorted[0];
        let btree = BTree{
            node: value,
            left: None,
            right: None,
        };
        return Some(Box::new(btree));
    }
    let mid_idx = sorted.len() / 2;
    let middle = sorted[mid_idx];
    let (left_side, _) = sorted.split_at(mid_idx);
    let left_tree = build_tree(left_side.to_vec());
    let (_, right_side) = sorted.split_at(mid_idx + 1);
    let right_tree = build_tree(right_side.to_vec());
    let btree = BTree{
        node: middle,
        left: left_tree,
        right: right_tree,
    };
    return Some(Box::new(btree));
}


fn main() {
    let mut rng = rand::rng();
    let mut v: Vec<i32> = (0..10)
        .map(|_| rng.random_range(0..1000))
        .collect();
    v.sort();
    let tree = build_tree(v);
    // let real_tree = tree.expect("Not Null");
    print_tree(&tree, 0);
    // println!("{:?}", real_tree);
}
