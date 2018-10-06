use ::chain::block::{Block};
use ::chain::chain::Chain;
use ::chain::chain_visitor::SumCipherTextVisitor;
use ::chain::chain_walker::{ChainWalker, LongestPathWalker};
use ::chain::transaction::Transaction;
use ::config::genesis::Genesis;
use ::p2p::codec::Message;
use bincode;
use crypto_rs::arithmetic::mod_int::ModInt;
use crypto_rs::el_gamal::ciphertext::CipherText;
use crypto_rs::el_gamal::encryption::encrypt;
use num::Zero;
use sha1::Sha1;
use std::{thread, time};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;

/// A protocol handler implements specific business logic
/// on what should be done when a message is received,
/// either from other running nodes or client applications.
pub trait ProtocolHandler {
    /// Handles a message received from another peer.
    /// The returned message is the direct response to the client
    /// from which we've received the provided message.
    ///
    /// - message: The message received from another node.
    fn handle(&mut self, message: Message) -> Message;

    /// Handles a message received on the RPC interface.
    /// Returns a pair of messages, whereas the first is meant to be sent
    /// to the client from which we are receiving the message, and the
    /// second is meant to be broadcast to all other known peers.
    ///
    /// - message: The message received from a client.
    fn handle_rpc(&mut self, message: Message) -> Option<(Message, Message)>;
}

/// The clique protocol provides a Proof-of-Authority (PoA)
/// sybil control mechanism.
#[derive(Serialize)]
pub struct CliqueProtocol {
    transactions: Vec<Transaction>,
    voting_information: VotingInformation,
    signer_index: usize,
    signer_count: usize,
    genesis: Genesis,
    chain: Chain,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct VotingInformation {
    pub total_votes: usize,
    pub cipher_text: CipherText,
}

impl CliqueProtocol {
    /// Create a new protocol instance.
    ///
    /// - own_address: The socket address the node is listening on. Used to find the own
    ///                sealer index in the genesis configuration.
    /// - genesis: The initial configuration of the clique protocol.
    ///
    /// # Panics
    /// Panics if the given own_address is not contained in the genesis configuration.
    pub fn new(own_address: SocketAddr, genesis: Genesis) -> Self {
        let own_signer_index = genesis.sealer.clone()
            .iter()
            .enumerate()
            .find(|&element| element.1.eq(&own_address.clone()))
            .expect("Could not find own socket address in sealers of genesis configuration")
            .0;

        trace!("Found own sealer index to be {} for own listening address {} in genesis configuration", own_signer_index.clone(), own_address.clone());

        let own_signer_count = genesis.sealer.len().clone();
        trace!("Found a total of {} sealer in genesis configuration", own_signer_count.clone());

        // Create a sha1 digest of the genesis configuration so that we can later
        // ensure, that we only accept blocks from a chain with the same configuration.
        let bytes = bincode::serialize(&genesis).unwrap();
        let digest: String = Sha1::from(bytes).hexdigest();

        let cipher_text = encrypt(&genesis.public_key.clone(), ModInt::zero());

        let voting_info = VotingInformation {
            total_votes: 0,
            cipher_text,
        };

        CliqueProtocol {
            transactions: vec![],
            voting_information: voting_info,
            signer_index: own_signer_index,
            signer_count: own_signer_count,
            genesis,
            chain: Chain::new(digest),
        }
    }

    /// Replace the own block chain with the given instance, if the given instance
    /// has a branch with a greater height than our longest branch.
    pub fn replace_chain(&mut self, chain: Chain) {
        let own_chain_height = self.chain.get_current_block_number();
        let other_chain_height = chain.get_current_block_number();

        if !chain.genesis_configuration_hash.eq(&self.chain.genesis_configuration_hash) {
            warn!("Not replacing chain {:?} as its genesis configuration does not match ours.", chain.clone());
            return;
        }

        trace!("My height: {}, other height: {}", own_chain_height, other_chain_height);

        if own_chain_height < other_chain_height {
            debug!("Replacing own chain of length {:?} with remote chain of length {:?}", own_chain_height, other_chain_height);
            self.chain = chain;
        }
    }

    /// Returns true, if the node is a leader in the current
    /// epoch and therefore allowed to sign blocks.
    fn is_leader(&self) -> bool {
        let current_block_number = self.chain.get_current_block_number();
        let expected_leader_index = current_block_number % self.signer_count;
        let am_i_leader = self.signer_index == expected_leader_index;

        trace!("Current block number is {}, expected leader is {}. Am I the leader? {}", current_block_number, expected_leader_index, am_i_leader);

        am_i_leader
    }

    /// Returns true, if the node is a co-leader in the current
    /// epoch and therefore allowed to sign a blocks after waiting for
    /// a particular wiggle time.
    fn is_co_leader(&self) -> bool {
        let current_block_number = self.chain.get_current_block_number();

        let lower_leader_index_bound = (current_block_number % self.signer_count) + 1;
        let upper_leader_index_bound = (current_block_number + self.genesis.clique.signer_limit) % self.signer_count;

        let am_i_co_leader = (self.signer_index >= lower_leader_index_bound) && (self.signer_index <= upper_leader_index_bound);

        trace!("Current block number is {}, leader index bound is [{}..{}]. Am I co-leader? {}", current_block_number, lower_leader_index_bound, upper_leader_index_bound, am_i_co_leader);

        am_i_co_leader
    }

    /// Handle a received transaction.
    fn on_transaction_receive(&mut self, transaction: Transaction) {
        if !transaction.is_valid(self.genesis.public_key.clone(), self.genesis.public_uciv.clone()) {
            warn!("Transaction {:?} is invalid. Not adding to chain.", transaction.clone());
            return;
        }

        if self.transactions.contains(&transaction) {
            trace!("Transaction {:?} is already contained. Not adding to chain", transaction.identifier.clone());
            return;
        }

        if self.is_leader() || self.is_co_leader() {
            debug!("We are either leader or co-leader and therefore adding transaction {:?} to buffer with current len {}", transaction.identifier.clone(), self.transactions.len());
            self.transactions.push(transaction);
        }
    }

    fn calculate_result(&self) -> VotingInformation {
        let mut sum_cipher_visitor = SumCipherTextVisitor::new(self.voting_information.cipher_text.clone());
        let longest_path_walker = LongestPathWalker::new();

        longest_path_walker.walk_chain(&self.chain, &mut sum_cipher_visitor);

        VotingInformation {
            cipher_text: sum_cipher_visitor.sum_cipher_text,
            total_votes: sum_cipher_visitor.total_votes,
        }
    }

    /// Sign a block with all current known transactions.
    /// May return None if a block with the same identifier is already contained
    /// in the chain of the node.
    pub fn sign(&mut self) -> Option<Block> {
        if !self.is_leader() && !self.is_co_leader() {
            trace!("Skipping to sign as neither leader nor co-leader");
            return None;
        }

        let now = SystemTime::now();
        let now_unix = now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

        let next_run = self.genesis.clique.block_period + self.chain.get_current_block_timestamp();

        if now_unix < next_run {
            trace!("Block period is not yet over. {:?} seconds left. Waiting...", next_run - now_unix);
            return None;
        }

        let current_block = self.chain.get_current_block();
        let block = Block::new(
            current_block.1.identifier.clone(),
            self.transactions.clone(),
        );

        if self.is_co_leader() {
            debug!("Signing as co-leader and therefore adding wiggle time before broadcast");
            // add some "wiggle" time to let leader nodes announce their blocks first
            let delay = time::Duration::from_millis(1000);

            thread::sleep(delay);

            // check whether we already received the block from the leader
            // -> no need to broadcast the block again
            if self.chain.blocks.contains_key(&block.identifier.clone()) {
                debug!("Skipping to broadcast block {:?} as already received from the leader.", block.identifier.clone());
                return None;
            }
        }

        // reset current state again
        self.transactions = vec![];

        // add block to our chain as well
        let is_added = self.chain.add_block(block.clone());

        if ! is_added {
            debug!("Block {} is already contained in the chain, possibly due to a leader broadcast earlier. Skipping broadcast.", block.identifier);
            let current_block_after_sign = self.chain.get_current_block();
            debug!("Current block without signing has height {:?} and identifier {:?}", current_block_after_sign.0, current_block_after_sign.1.identifier);

            return None;
        }

        let current_block_after_sign = self.chain.get_current_block();
        debug!("Current block after signing has height {:?} and identifier {:?}", current_block_after_sign.0, current_block_after_sign.1.identifier);

        Some(block)
    }
}

impl ProtocolHandler for CliqueProtocol {
    fn handle(&mut self, message: Message) -> Message {
        match message {
            Message::None => Message::None,
            Message::Ping => Message::Pong,
            Message::Pong => Message::None,
            Message::TransactionPayload(transaction) => {
                // if we received the transaction from another node
                // there is no need to broadcast it again, as this
                // was the task of the node from which we've received it.
                self.on_transaction_receive(transaction);

                Message::TransactionAccept
            }
            Message::TransactionAccept => Message::None,
            Message::BlockRequest(_) => unimplemented!("Not yet implemented: Return block requested"),
            Message::BlockPayload(block) => {

                // check whether we have the contained transaction already in our buffer
                let mut non_duplicate_trxs = vec![];
                for transaction in block.data.transactions.clone() {
                    if ! self.transactions.contains(&transaction.clone()) {
                        non_duplicate_trxs.push(transaction.clone());
                    }
                }

                // TODO: is it wrong to remove all transactions from the buffer?
                //       -> they are already broadcast to other nodes, possibly the leader
                self.transactions = non_duplicate_trxs;

                let is_added = self.chain.add_block(block);

                if is_added {
                    return Message::BlockAccept;
                }

                Message::BlockDuplicated
            }
            Message::BlockAccept => Message::None,
            Message::BlockDuplicated => Message::None,
            Message::ChainRequest => Message::ChainResponse(self.chain.clone()),
            Message::ChainResponse(chain) => {
                self.replace_chain(chain);

                Message::ChainAccept
            }
            Message::ChainAccept => Message::None,
            Message::OpenVote => {
                self.on_transaction_receive(Transaction::new_voting_opened());

                Message::OpenVoteAccept
            },
            Message::OpenVoteAccept => Message::None,
            Message::CloseVote => {
                self.on_transaction_receive(Transaction::new_voting_closed());

                Message::CloseVoteAccept
            },
            Message::CloseVoteAccept => Message::None,
            Message::RequestTally => Message::None,
            Message::RequestTallyPayload(_) => Message::None,
        }
    }

    fn handle_rpc(&mut self, message: Message) -> Option<(Message, Message)> {
        match message {
            Message::None => None,
            Message::Ping => None,
            Message::Pong => None,
            Message::TransactionPayload(transaction) => {
                // we've received the transaction from a client,
                // which means that we have to add it to our set of known
                // transactions (in case we are a co-/leader) and then
                // notify all other nodes in the network about this new transaction.
                self.on_transaction_receive(transaction.clone());

                Some((Message::TransactionAccept, Message::TransactionPayload(transaction)))
            }
            Message::TransactionAccept => None,
            Message::BlockRequest(_) => None,
            Message::BlockPayload(_) => None,
            Message::BlockAccept => None,
            Message::BlockDuplicated => None,
            Message::ChainRequest => Some((Message::ChainResponse(self.chain.clone()), Message::None)),
            Message::ChainResponse(_) => None,
            Message::ChainAccept => None,
            // TODO: add flag to chain
            Message::OpenVote => {
                self.on_transaction_receive(Transaction::new_voting_opened());

                Some((Message::OpenVoteAccept, Message::OpenVote))
            },
            Message::OpenVoteAccept => None,
            // TODO: add flag to chain
            Message::CloseVote => {
                self.on_transaction_receive(Transaction::new_voting_closed());

                Some((Message::CloseVoteAccept, Message::CloseVote))
            },
            Message::CloseVoteAccept => None,
            Message::RequestTally => {
                let final_tally = self.calculate_result();

                Some((Message::RequestTallyPayload(final_tally), Message::None))
            }
            Message::RequestTallyPayload(_) => None,
        }
    }
}