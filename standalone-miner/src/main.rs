use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

// Define the necessary structures locally to avoid dependencies on the broken code
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: Vec::new(),
        }
    }
    
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: String,
    pub nonce: u64,
    pub fractal: FractalTriangle,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, prev_hash: String, nonce: u64, fractal: FractalTriangle, hash: String) -> Self {
        Block { index, timestamp, prev_hash, nonce, fractal, hash }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FractalTriangle {
    pub depth: usize,
    pub vertices: Vec<(f64, f64)>,
}

impl FractalTriangle {
    pub fn generate(depth: usize) -> Self {
        let vertices = vec![(0.0, 0.0), (1.0, 0.0), (0.5, 0.866)];
        FractalTriangle { depth, vertices }
    }
}

#[derive(Debug)]
pub struct Miner {
    pub address: String,
    pub hash_rate: u64,
}

// Simple SHA256 implementation for hashing
pub fn hash_with_fractal_salt(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(salt);
    hasher.finalize().to_vec()
}

pub fn get_target(difficulty: u32) -> Vec<u8> {
    let mut target = vec![0u8; 32];
    let bytes = difficulty as usize / 8;
    for i in 0..bytes {
        target[i] = 0;
    }
    if bytes < 32 {
        target[bytes] = 255 >> (difficulty % 8);
    }
    target
}

pub fn validate_proof(hash: &[u8], target: &[u8]) -> bool {
    for (h, t) in hash.iter().zip(target.iter()) {
        if h < t {
            return true;
        } else if h > t {
            return false;
        }
    }
    true
}

impl Miner {
    pub fn mine(&self, data: &[u8], salt: &[u8], difficulty: u32) -> (u64, Vec<u8>) {
        let target = get_target(difficulty);
        let mut nonce: u64 = 0;
        loop {
            let mut input = Vec::from(data);
            input.extend_from_slice(&nonce.to_le_bytes());
            let hash = hash_with_fractal_salt(&input, salt);
            if validate_proof(&hash, &target) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    println!("Mining the first block...");
    
    // Initialize blockchain
    let mut blockchain = Blockchain::new();
    
    // Check if blockchain already has blocks
    if !blockchain.chain.is_empty() {
        println!("Blockchain already has blocks. First block height: {}", blockchain.chain.len());
        return;
    }
    
    // Create miner
    let miner = Miner {
        address: "genesis-miner".to_string(),
        hash_rate: 1000,
    };
    
    // Create fractal for the genesis block
    let fractal = FractalTriangle::generate(1);
    
    // Create initial data for the block
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    // Create block data to mine
    let mut block_data = Vec::new();
    block_data.extend_from_slice(&0u64.to_le_bytes()); // index
    block_data.extend_from_slice(&timestamp.to_le_bytes()); // timestamp
    block_data.extend_from_slice(b"0"); // previous hash (genesis block)
    
    // Mine the block
    println!("Starting mining process...");
    let difficulty = 2; // Low difficulty for testing
    let (nonce, hash) = miner.mine(&block_data, b"genesis-salt", difficulty);
    
    // Create the genesis block
    let genesis_block = Block::new(
        0, // index
        timestamp,
        "0".to_string(), // previous hash
        nonce,
        fractal,
        format!("{:x?}", hash) // Convert hash to hex string
    );
    
    // Add block to blockchain
    blockchain.add_block(genesis_block);
    
    println!("First block mined successfully!");
    println!("Block height: {}", blockchain.chain.len());
    println!("Block hash: {:?}", hash);
    println!("Nonce: {}", nonce);
}
