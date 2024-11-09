use methods::{MERKLE_GUEST_ELF, MERKLE_GUEST_ID};
use rand::Rng;
use risc0_zkvm::{default_prover, ExecutorEnv};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MerkleNode {
    key: Option<i32>,
    value: Option<char>,
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    parent: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(key: i32, value: char) -> Self {
        let data = format!("{}{}", key, value);
        let hash = Sha256::digest(data.as_bytes()).to_vec();
        MerkleNode {
            key: Some(key),
            value: Some(value),
            hash,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn from_children(mut left: MerkleNode, mut right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let hash = hasher.finalize().to_vec();

        let parent_node = MerkleNode {
            key: None,
            value: None,
            hash,
            left: Some(Box::new(left.clone())),
            right: Some(Box::new(right.clone())),
            parent: None,
        };

        left.parent = Some(Box::new(parent_node.clone()));
        right.parent = Some(Box::new(parent_node.clone()));

        parent_node
    }
}

#[derive(Debug)]
struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: HashMap<i32, MerkleNode>,
}

impl MerkleTree {
    fn new() -> Self {
        MerkleTree {
            root: None,
            leaves: HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: char) {
        let node = MerkleNode::new(key, value);
        self.leaves.insert(key, node);
        self.build_tree();
    }

    fn build_tree(&mut self) {
        let mut nodes: Vec<MerkleNode> = self.leaves.values().cloned().collect();

        while nodes.len() > 1 {
            let mut temp_nodes = vec![];

            for i in (0..nodes.len()).step_by(2) {
                if i + 1 < nodes.len() {
                    let parent = MerkleNode::from_children(nodes[i].clone(), nodes[i + 1].clone());
                    temp_nodes.push(parent);
                } else {
                    temp_nodes.push(nodes[i].clone());
                }
            }
            nodes = temp_nodes;
        }

        self.root = nodes.into_iter().next();
    }

    fn get(&self, key: i32) -> char {
        self.leaves
            .get(&key)
            .and_then(|node| node.value)
            .unwrap_or('_')
    }

    fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash.clone())
    }

    fn get_path_to_root(&self, key: i32) -> Vec<Vec<u8>> {
        let mut path = Vec::new();
        let mut current_node = self.leaves.get(&key);

        while let Some(node) = current_node {
            path.push(node.hash.clone());
            current_node = node.parent.as_deref();
        }
        path
    }

    fn print_all_nodes(&self) {
        if let Some(root) = &self.root {
            println!("Root hash: {:?}", hex::encode(&root.hash));
            Self::print_node_recursive(root, 0);
        } else {
            println!("Merkle Tree is empty.");
        }
    }

    fn print_node_recursive(node: &MerkleNode, depth: usize) {
        if let (Some(key), Some(value)) = (node.key, node.value) {
            println!(
                "{}Leaf at depth {}: Key = {}, Value = {}, Hash = {:?}",
                "  ".repeat(depth),
                depth,
                key,
                value,
                hex::encode(&node.hash)
            );
        } else {
            println!(
                "{}Node at depth {}: Hash = {:?}",
                "  ".repeat(depth),
                depth,
                hex::encode(&node.hash)
            );
        }

        if let Some(left) = &node.left {
            Self::print_node_recursive(left, depth + 1);
        }
        if let Some(right) = &node.right {
            Self::print_node_recursive(right, depth + 1);
        }
    }
}

fn int_to_char(n: i32) -> char {
    let alphabet_size = 26;
    let base_char = 'A' as u8;
    let char_code = base_char + ((n - 1) % alphabet_size) as u8;
    char_code as char
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    /*
    // Create a sample merkle tree
    let mut merkle_tree = MerkleTree::new();
    for k in 1..101 {
    let v = int_to_char(k);
    merkle_tree.insert(k, v);
    }

    merkle_tree.print_all_nodes();

    let mut rng = rand::thread_rng();

    let random1: i32 = rng.gen_range(1..=100); // Generates a random number between 1 and 100
    let random2: i32 = rng.gen_range(1..=100);
    let random3: i32 = rng.gen_range(1..=100);

    println!("Random keys: {}, {}, {}", random1, random2, random3);

    // Fetch the values and compute the performance metrics
    let value1: char = merkle_tree.get(random1);
    let value2: char = merkle_tree.get(random2);
    let value3: char = merkle_tree.get(random3);
    */

    let mut merkle_tree1 = MerkleTree::new();
    for k in 1..101 {
        let v = int_to_char(k);
        merkle_tree1.insert(k, v);
    }
    merkle_tree1.print_all_nodes();

    let mut merkle_tree2 = MerkleTree::new();
    for k in 1..101 {
        let v = int_to_char(k + 1);
        merkle_tree2.insert(k, v);
    }

    let mut merkle_tree3 = MerkleTree::new();
    for k in 1..101 {
        let v = int_to_char(k + 2);
        merkle_tree3.insert(k, v);
    }

    let mut rng = rand::thread_rng();

    let rand: i32 = rng.gen_range(1..=100); // Generates a random number between 1 and 100
    println!("Random key: {}", rand);

    let value1: char = merkle_tree1.get(rand);
    let value2: char = merkle_tree2.get(rand);
    let value3: char = merkle_tree3.get(rand);

    let input: [char; 3] = [value1, value2, value3];

    println!("Values: {}, {}, {}", input[0], input[1], input[2]);

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let prove_info = prover.prove(env, MERKLE_GUEST_ELF).unwrap();

    let receipt = prove_info.receipt;

    // TODO: output the receipt into a file and create a verifier

    let journal: String = receipt.journal.decode().unwrap();

    receipt.verify(MERKLE_GUEST_ID).unwrap();

    println!("Verification passed!");
    println!("Journal: {}", journal);
}
