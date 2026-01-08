use chrono::{DateTime, Utc};

use crate::u256;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
        }
    }

    pub fn hash(&self) -> ! {
        todo!()
    }
}

pub struct BlockHeader {
    /// Previous block's hash
    pub previous_block_hash: [u8; 32],
    /// Merkle root hash of the transactions
    pub merkle_root_hash: [u8; 32],
    /// Timestamp of the block
    pub timestamp: DateTime<Utc>,
    /// Nonce used for mining
    pub nonce: u64,
    /// Target difficulty
    pub target: u256,
}

impl BlockHeader {
    pub fn new(
        previous_block_hash: [u8; 32],
        merkle_root_hash: [u8; 32],
        timestamp: DateTime<Utc>,
        nonce: u64,
        target: u256,
    ) -> Self {
        Self {
            previous_block_hash,
            merkle_root_hash,
            timestamp,
            nonce,
            target,
        }
    }

    pub fn hash(&self) -> ! {
        todo!()
    }
}

pub struct TxInput {
    pub previous_output_hash: [u8; 32],
    pub signature: [u8; 64],
}

pub struct TxOutput {
    pub value: u64,
    pub unique_id: uuid::Uuid,
    pub public_key: [u8; 33],
}

pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self { inputs, outputs }
    }

    pub fn hash(&self) -> ! {
        todo!()
    }
}
