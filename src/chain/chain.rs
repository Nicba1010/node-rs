use std::collections::HashMap;
use std::vec::Vec;
use bincode;
use sha1::Sha1;

use ::chain::block::Block;
use ::chain::transaction::Transaction;
use ::config::genesis::Genesis;
use chain::chain_visitor::HeaviestBlockVisitor;
use chain::chain_walker::LongestPathWalker;
use chain::chain_walker::ChainWalker;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Chain {
    /// the hash of the genesis configuration
    pub genesis_configuration_hash: String,
    /// the hash of the genesis block
    pub genesis_identifier_hash: String,
    /// all known blocks
    pub blocks: HashMap<String, Block>,
    /// a matrix creating the relation between blocks
    pub adjacent_matrix: HashMap<String, Vec<String>>
}

impl Chain {

    pub fn new(genesis: Genesis) -> Self {
        // create the genesis block with an empty hash and no transactions
        let trxs: Vec<Transaction> = vec![];
        let genesis_block: Block = Block::new(String::new(), trxs);

        let mut blocks = HashMap::new();
        blocks.insert(genesis_block.identifier.clone(), genesis_block.clone());

        // Add an entry for the genesis block in the adjacent matrix,
        // i.e. initialize children of the genesis block as an empty vector.
        let mut adjacent_matrix: HashMap<String, Vec<String>> = HashMap::new();
        adjacent_matrix.insert(genesis_block.identifier.clone(), vec![]);

        // Create a sha1 digest of the genesis configuration so that we can later
        // ensure, that we only accept blocks from a chain with the same configuration.
        let bytes = bincode::serialize(&genesis).unwrap();
        let digest: String = Sha1::from(bytes).hexdigest();

        trace!("Genesis block hash is: {:?}", genesis_block.identifier.clone());

        Chain {
            genesis_configuration_hash: digest,
            genesis_identifier_hash: genesis_block.identifier.clone(),
            blocks,
            adjacent_matrix
        }
    }

    pub fn get_current_block_number(&self) -> usize {
        self.get_current_block().0
    }

    pub fn get_current_block_timestamp(&self) -> u64 {
        self.get_current_block().1.data.timestamp
    }

    pub fn get_current_block(&self) -> (usize, Block) {
        let mut heaviest_block_visitor = HeaviestBlockVisitor::new();
        let longest_path_walker = LongestPathWalker::new();
        longest_path_walker.visit_chain(&self, &mut heaviest_block_visitor);

        let heaviest_block_height_option = heaviest_block_visitor.height;
        assert!(heaviest_block_height_option.is_some());
        let heaviest_block_height = heaviest_block_height_option.unwrap();

        let option = heaviest_block_visitor.heaviest_block;
        assert!(option.is_some());
        let heaviest_block_reference = option.unwrap();

        (heaviest_block_height, (*self.blocks.get(&heaviest_block_reference).unwrap()).clone())
    }

    /// Returns true, if the parent of the given block exists, false otherwise.
    pub fn has_parent_of_block(self, block: Block) -> bool {
        let parent_block = self.adjacent_matrix.get(&block.data.parent);

        parent_block.is_some()
    }

    /// Add the block as child to its corresponding parent.
    /// Panics, if the parent block specified does not exist.
    /// Therefore, invoke `has_parent_of_block` first.
    pub fn add_block(&mut self, block: Block) {
        // add block hash to its parent as child
        self.adjacent_matrix
            // in-place modification of the vector
            .entry(block.data.parent.clone())
            .and_modify(|parent_block_children| {
                if ! parent_block_children.contains(&block.identifier.clone()) {
                    info!("Adding block {:?} containing {:?} transactions to chain.", block.identifier.clone(), block.data.transactions.len());
                    parent_block_children.push(block.identifier.clone());
                } else {
                    trace!("Not adding block {:?} as it is already contained.", block.identifier.clone());
                }
            });

        // add a new entry for the block we've inserted
        // having currently no children
        self.adjacent_matrix
            // in-place modification of the vector
            .entry(block.identifier.clone())
            .or_insert(vec![]);

        // insert the block finally
        self.blocks.insert(block.identifier.clone(), block);
    }
}

#[cfg(test)]
mod chain_test {

    use ::config::genesis::{CliqueConfig, Genesis};
    use ::chain::block::{Block, BlockContent};
    use ::chain::chain::Chain;

    #[test]
    fn test_add_duplicate_block() {
        let genesis = Genesis {
            version: "test_version".to_string(),
            clique: CliqueConfig {
                block_period: 10,
                signer_limit: 1
            },
            sealer: vec![]
        };


        let mut chain = Chain::new(genesis);
        let genesis_id = chain.genesis_identifier_hash.clone();

        let block = Block {
            identifier: "1".to_string(),
            data: BlockContent {
                parent: genesis_id.clone(),
                timestamp: 1,
                transactions: vec![]
            }
        };

        assert!(chain.blocks.len().eq(&1));

        // first level
        chain.add_block(block.clone());
        chain.add_block(block.clone());

        // genesis block and the first of the duplicates
        assert!(chain.blocks.len().eq(&2));

        // assert that adjacent matrix is also correct:
        // i.e. only one child is present for the genesis block
        assert!(chain.adjacent_matrix.get(&genesis_id.clone()).unwrap().len().eq(&1));
    }

}