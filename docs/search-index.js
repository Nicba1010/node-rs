var searchIndex = {};
searchIndex["node_rs"] = {"doc":"`node_rs` provides a binary to run a binary vote on a permissioned blockchain.","items":[[0,"chain","node_rs","Holds all functionality related to the blockchain itself.",null,null],[0,"block","node_rs::chain","A block of the blockchain.",null,null],[3,"BlockContent","node_rs::chain::block","The content of a block. All contained fields are hashed and represent the identifier of the block.",null,null],[12,"parent","","",0,null],[12,"timestamp","","",0,null],[12,"transactions","","",0,null],[3,"Block","","A block containing the identifier as well as its content, building up the identifier.",null,null],[12,"identifier","","",1,null],[12,"data","","",1,null],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"blockcontent"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"blockcontent"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"blockcontent"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"block"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"self"},{"name":"block"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"block"}}],[11,"new","","Create a new block with the given parameters:",1,{"inputs":[{"name":"string"},{"generics":["transaction"],"name":"vec"}],"output":{"name":"self"}}],[0,"chain","node_rs::chain","The data structure of the blockchain.",null,null],[3,"Chain","node_rs::chain::chain","",null,null],[12,"genesis_configuration_hash","","the hash of the genesis configuration",2,null],[12,"genesis_identifier_hash","","the hash of the genesis block",2,null],[12,"blocks","","all known blocks",2,null],[12,"adjacent_matrix","","a matrix creating the relation between blocks key is the parent, values are its children",2,null],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"chain"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"chain"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"chain"}}],[11,"new","","",2,{"inputs":[{"name":"string"}],"output":{"name":"self"}}],[11,"get_current_block_number","","",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"get_current_block_timestamp","","",2,{"inputs":[{"name":"self"}],"output":{"name":"u64"}}],[11,"get_current_block","","",2,null],[11,"has_parent_of_block","","Returns true, if the parent of the given block exists, false otherwise.",2,{"inputs":[{"name":"self"},{"name":"block"}],"output":{"name":"bool"}}],[11,"add_block","","Add the block as child to its corresponding parent. Panics, if the parent block specified does not exist. Therefore, invoke `has_parent_of_block` first.",2,{"inputs":[{"name":"self"},{"name":"block"}],"output":{"name":"bool"}}],[0,"chain_visitor","node_rs::chain","Visitors of the chain which can be used in combination with a chain walker.",null,null],[3,"FindTransactionVisitor","node_rs::chain::chain_visitor","This visitor expects to be called on each level in order to find a transaction with a particular identifier.",null,null],[3,"HeaviestBlockVisitor","","This visitor expects to be called exactly once with the heaviest block in the chain.",null,null],[12,"height","","The height of the heaviest block.",3,null],[12,"heaviest_block","","The hash of the string once it is assigned, or None, if this visitor was never visited.",3,null],[3,"SumCipherTextVisitor","","Sums up all votes contained in the transactions, after the voting has been opened and until it is closed again.",null,null],[8,"ChainVisitor","","",null,null],[10,"visit_block","","Visit a particular block",4,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"block"}],"output":null}],[11,"new","","Create a new find transaction visitor",5,{"inputs":[{"name":"string"}],"output":{"name":"findtransactionvisitor"}}],[11,"get_found_transaction","","Get the found transaction. Returns None if the transaction could not be found, the transaction otherwise.",5,{"inputs":[{"name":"self"}],"output":{"generics":["transaction"],"name":"option"}}],[11,"visit_block","","Visit a block of the blockchain.",5,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"block"}],"output":null}],[11,"new","","Create a new `HeaviestBlockVisitor` having a `None` hash of the heaviest block.",3,{"inputs":[],"output":{"name":"heaviestblockvisitor"}}],[11,"visit_block","","Expects to be called only once. Will panic otherwise.",3,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"block"}],"output":null}],[11,"new","","",6,{"inputs":[{"name":"publickey"}],"output":{"name":"sumciphertextvisitor"}}],[11,"get_votes","","",6,null],[11,"visit_block","","",6,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"block"}],"output":null}],[0,"chain_walker","node_rs::chain","`ChainWalker`s walk the blockchain in a specific, implementation-defined order. Each such walker is provided with a `ChainVisitor` which in turn is invoked by the walker at crucial steps during the traversal of the chain. When and if a visitor is invoked is specific to an implementation of a `ChainWalker`.",null,null],[3,"HeaviestBlockWalker","node_rs::chain::chain_walker","The heaviest block walker walks the given chain to find the deepest block currently known and invokes any provided visitor with the block found at the end of the longest path.",null,null],[3,"LongestPathWalker","","",null,null],[8,"ChainWalker","","A ChainWalker walks the given chain in a particular order and can invoke the given visitor at any point during its traversal.",null,null],[10,"walk_chain","","Visit the given chain in a particular order. The concrete implementation will specify when and for which blocks the provided visitor should be invoked.",7,{"inputs":[{"name":"self"},{"name":"chain"},{"name":"f"}],"output":null}],[11,"new","","",8,{"inputs":[],"output":{"name":"heaviestblockwalker"}}],[11,"walk_chain","","Visits the given chain to find the deepest block in the chain, i.e. the one having the most parents. Once found, it will invoke the given visitor with the corresponding found block.",8,{"inputs":[{"name":"self"},{"name":"chain"},{"name":"f"}],"output":null}],[11,"new","","",9,{"inputs":[],"output":{"name":"longestpathwalker"}}],[11,"walk_chain","","",9,{"inputs":[{"name":"self"},{"name":"chain"},{"name":"f"}],"output":null}],[0,"transaction","node_rs::chain","A transaction of the blockchain.",null,null],[3,"TransactionData","node_rs::chain::transaction","",null,null],[12,"voter_idx","","",10,null],[12,"cipher_text","","",10,null],[12,"membership_proof","","",10,null],[12,"cai_proof","","",10,null],[3,"Transaction","","Use Deserialize from Serde, Hash from std::hash",null,null],[12,"identifier","","",11,null],[12,"trx_type","","",11,null],[12,"data","","",11,null],[4,"TransactionType","","",null,null],[13,"Vote","","",12,null],[13,"VoteOpened","","",12,null],[13,"VoteClosed","","",12,null],[11,"eq","","",12,{"inputs":[{"name":"self"},{"name":"transactiontype"}],"output":{"name":"bool"}}],[11,"hash","","",12,null],[11,"clone","","",12,{"inputs":[{"name":"self"}],"output":{"name":"transactiontype"}}],[11,"fmt","","",12,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",10,{"inputs":[{"name":"self"},{"name":"transactiondata"}],"output":{"name":"bool"}}],[11,"ne","","",10,{"inputs":[{"name":"self"},{"name":"transactiondata"}],"output":{"name":"bool"}}],[11,"hash","","",10,null],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",10,{"inputs":[{"name":"self"}],"output":{"name":"transactiondata"}}],[11,"hash","","",11,null],[11,"fmt","","",11,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",11,{"inputs":[{"name":"self"}],"output":{"name":"transaction"}}],[11,"new_voting_opened","","",11,{"inputs":[],"output":{"name":"transaction"}}],[11,"new_voting_closed","","",11,{"inputs":[],"output":{"name":"transaction"}}],[11,"new_vote","","",11,{"inputs":[{"name":"usize"},{"name":"ciphertext"},{"name":"membershipproof"},{"name":"caiproof"}],"output":{"name":"transaction"}}],[11,"is_valid","","Verify whether the proofs submitted along with the transaction are valid with respect to the proofs submitted along with it.",11,{"inputs":[{"name":"self"},{"name":"publickey"},{"generics":["imageset"],"name":"vec"}],"output":{"name":"bool"}}],[11,"eq","","",11,{"inputs":[{"name":"self"},{"name":"transaction"}],"output":{"name":"bool"}}],[0,"config","node_rs","Holds all functionality related to the blockchain configuration, e.g. Genesis.",null,null],[0,"genesis","node_rs::config","",null,null],[3,"GenesisData","node_rs::config::genesis","Use Deserialize from Serde, Hash from std::hash",null,null],[12,"version","","",13,null],[12,"clique","","",13,null],[12,"sealer","","",13,null],[3,"CliqueConfig","","A configuration element for clique specific values.",null,null],[12,"block_period","","",14,null],[12,"signer_limit","","",14,null],[3,"Genesis","","The configuration for the blockchain, usually included in the first block of a chain, and therefore often referred to as genesis block.",null,null],[12,"version","","",15,null],[12,"clique","","",15,null],[12,"sealer","","",15,null],[12,"public_key","","",15,null],[12,"public_uciv","","",15,null],[11,"fmt","","",13,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",14,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","Create a new Genesis configuration based on a specific configuration.",15,{"inputs":[{"name":"str"},{"name":"str"},{"name":"str"}],"output":{"name":"self"}}],[0,"p2p","node_rs","Holds all functionality related to the networking stuff.",null,null],[0,"thread","node_rs::p2p","Multi-threading functionality is here. Contains a Threadpool among otther things.",null,null],[3,"ThreadPool","node_rs::p2p::thread","",null,null],[11,"new","","Create a new ThreadPool.",16,{"inputs":[{"name":"usize"}],"output":{"name":"threadpool"}}],[11,"execute","","",16,{"inputs":[{"name":"self"},{"name":"f"}],"output":null}],[11,"drop","","",16,{"inputs":[{"name":"self"}],"output":null}],[0,"node","node_rs::p2p","A node of the blockchain. This is where listening and broadcasting happens.",null,null],[3,"Node","node_rs::p2p::node","Forms a node in the blockchain.",null,null],[11,"new","","Creates a new node.",17,{"inputs":[{"name":"socketaddr"},{"name":"socketaddr"},{"name":"genesis"}],"output":{"name":"node"}}],[11,"listen","","Start a listener on the bootstrap address.",17,{"inputs":[{"name":"self"}],"output":null}],[11,"listen_rpc","","Start to listen for incoming RPC connections, i.e. connections from an end-user client. Compared to `pub fn listen(&self)`, incoming messages may be handled a bit differently.",17,{"inputs":[{"name":"self"}],"output":null}],[11,"request_chain_copy","","Send a request for a copy of the blockchain to all known nodes.",17,{"inputs":[{"name":"self"}],"output":null}],[11,"sign","","Start the main loop to sign (aka. mint) blocks in the network.",17,{"inputs":[{"name":"self"}],"output":null}],[0,"codec","node_rs::p2p","The codec definition used to send information between nodes.",null,null],[3,"JsonCodec","node_rs::p2p::codec","JsonCodec is able to encode and decode a particular `Message` as a json `String` and vice-versa, respectively.",null,null],[4,"Message","","Messages used to communicate information between nodes.",null,null],[13,"Ping","","",18,null],[13,"Pong","","",18,null],[13,"TransactionPayload","","",18,null],[13,"TransactionAccept","","",18,null],[13,"BlockRequest","","",18,null],[13,"BlockPayload","","",18,null],[13,"BlockAccept","","",18,null],[13,"BlockDuplicated","","",18,null],[13,"ChainRequest","","",18,null],[13,"ChainResponse","","",18,null],[13,"ChainAccept","","",18,null],[13,"OpenVote","","",18,null],[13,"OpenVoteAccept","","",18,null],[13,"CloseVote","","",18,null],[13,"CloseVoteAccept","","",18,null],[13,"RequestTally","","",18,null],[13,"RequestTallyPayload","","",18,null],[13,"FindTransaction","","",18,null],[13,"FindTransactionResponse","","",18,null],[13,"None","","",18,null],[8,"Codec","","A codec is able to encode as well decode a particular `Message` into a corresponding `String` representation.",null,null],[10,"encode","","Encode the given message into a string.",19,{"inputs":[{"name":"message"}],"output":{"name":"string"}}],[10,"decode","","Decode the given string into a message.",19,{"inputs":[{"name":"string"}],"output":{"name":"message"}}],[11,"eq","","",18,{"inputs":[{"name":"self"},{"name":"message"}],"output":{"name":"bool"}}],[11,"ne","","",18,{"inputs":[{"name":"self"},{"name":"message"}],"output":{"name":"bool"}}],[11,"clone","","",18,{"inputs":[{"name":"self"}],"output":{"name":"message"}}],[11,"fmt","","",18,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","Encode the given message into a JSON string. If the message cannot be encoded, an empty string will be returned.",20,{"inputs":[{"name":"message"}],"output":{"name":"string"}}],[11,"decode","","Decode the given JSON string into a corresponding Message. Will return a `Message::None` if the string cannot be decoded.",20,{"inputs":[{"name":"string"}],"output":{"name":"message"}}],[0,"protocol","node_rs","Holds all functionality related to the protocol used to communicate blocks and transactions.",null,null],[0,"clique","node_rs::protocol","A simplified version of the Clique protocol where voting on network participants is omitted.",null,null],[3,"CliqueProtocol","node_rs::protocol::clique","The clique protocol provides a Proof-of-Authority (PoA) sybil control mechanism.",null,null],[3,"Tally","","Holds the tally of the voting.",null,null],[12,"total_votes","","",21,null],[12,"cipher_text","","",21,null],[8,"ProtocolHandler","","A protocol handler implements specific business logic on what should be done when a message is received, either from other running nodes or client applications.",null,null],[10,"handle","","Handles a message received from another peer. The returned message is the direct response to the client from which we've received the provided message.",22,{"inputs":[{"name":"self"},{"name":"message"}],"output":{"name":"message"}}],[10,"handle_rpc","","Handles a message received on the RPC interface. Returns a pair of messages, whereas the first is meant to be sent to the client from which we are receiving the message, and the second is meant to be broadcast to all other known peers.",22,{"inputs":[{"name":"self"},{"name":"message"}],"output":{"name":"option"}}],[11,"eq","","",21,{"inputs":[{"name":"self"},{"name":"tally"}],"output":{"name":"bool"}}],[11,"ne","","",21,{"inputs":[{"name":"self"},{"name":"tally"}],"output":{"name":"bool"}}],[11,"fmt","","",21,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",21,{"inputs":[{"name":"self"}],"output":{"name":"tally"}}],[11,"new","","Create a new protocol instance.",23,{"inputs":[{"name":"socketaddr"},{"name":"genesis"}],"output":{"name":"self"}}],[11,"replace_chain","","Replace the own block chain with the given instance, if the given instance has a branch with a greater height than our longest branch.",23,{"inputs":[{"name":"self"},{"name":"chain"}],"output":null}],[11,"is_leader","","Returns true, if the node is a leader in the current epoch and therefore allowed to sign blocks.",23,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_co_leader","","Returns true, if the node is a co-leader in the current epoch and therefore allowed to sign a blocks after waiting for a particular wiggle time.",23,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_block_period_over","","",23,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"create_current_block_and_reset_transaction_buffer","","",23,{"inputs":[{"name":"self"}],"output":{"name":"block"}}],[11,"reset_transaction_buffer","","",23,{"inputs":[{"name":"self"}],"output":null}],[11,"sign","","Sign a block with all current known transactions. May return None if a block with the same identifier is already contained in the chain of the node.",23,{"inputs":[{"name":"self"},{"name":"block"}],"output":{"generics":["block"],"name":"option"}}],[11,"handle","","",23,{"inputs":[{"name":"self"},{"name":"message"}],"output":{"name":"message"}}],[11,"handle_rpc","","",23,{"inputs":[{"name":"self"},{"name":"message"}],"output":{"name":"option"}}]],"paths":[[3,"BlockContent"],[3,"Block"],[3,"Chain"],[3,"HeaviestBlockVisitor"],[8,"ChainVisitor"],[3,"FindTransactionVisitor"],[3,"SumCipherTextVisitor"],[8,"ChainWalker"],[3,"HeaviestBlockWalker"],[3,"LongestPathWalker"],[3,"TransactionData"],[3,"Transaction"],[4,"TransactionType"],[3,"GenesisData"],[3,"CliqueConfig"],[3,"Genesis"],[3,"ThreadPool"],[3,"Node"],[4,"Message"],[8,"Codec"],[3,"JsonCodec"],[3,"Tally"],[8,"ProtocolHandler"],[3,"CliqueProtocol"]]};
initSearch(searchIndex);
