use crate::{block::Block, state::State};
use commit::{Commitment, Committable};
use hotshot::traits::{
    election::{
        static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
        vrf::JfPubKey,
    },
    implementations::{MemoryNetwork, MemoryStorage},
    NodeImplementation,
};
use hotshot_types::{
    data::ViewNumber,
    traits::{block_contents::Transaction as HotShotTransaction, node_implementation::NodeTypes},
};
use jf_primitives::signatures::BLSSignatureScheme;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;

mod block;
mod state;

#[derive(Debug, Clone)]
struct Node;

impl NodeImplementation<SeqTypes> for Node {
    type Storage = MemoryStorage<SeqTypes>;

    type Networking = MemoryNetwork<SeqTypes>;

    type Election = StaticCommittee<SeqTypes>;
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
struct SeqTypes;

type Param381 = ark_bls12_381::Parameters;
type SignatureSchemeType = BLSSignatureScheme<Param381>;
type SignatureKeyType = JfPubKey<SignatureSchemeType>;

impl NodeTypes for SeqTypes {
    type Time = ViewNumber;

    type BlockType = Block;

    type SignatureKey = SignatureKeyType;

    type VoteTokenType = StaticVoteToken<SignatureKeyType>;

    type Transaction = SequencerTransaction;

    type ElectionConfigType = StaticElectionConfig;

    type StateType = State;
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    // TODO: Can we nest these errors in a `ValidationError` to group them?

    // Parent state commitment of block doesn't match current state commitment
    IncorrectParent,

    // New view number isn't strictly after current view
    IncorrectView,

    // Genesis block either has zero or more than one transaction
    GenesisWrongSize,

    // Genesis transaction not present in genesis block
    MissingGenesis,

    // Genesis transaction in non-genesis block
    UnexpectedGenesis,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct VmId(u64);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Transaction {
    vm: VmId,
    payload: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct ApplicationTransaction(Vec<u8>);

trait Vm {
    type Transaction: DeserializeOwned + Serialize + Sync + Send;
    fn id() -> VmId;
}

#[derive(Clone, Debug)]
struct TestVm;

impl Vm for TestVm {
    type Transaction = ApplicationTransaction;
    fn id() -> VmId {
        VmId(0)
    }
}

impl HotShotTransaction for SequencerTransaction {}

impl Committable for Transaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Transaction")
            .u64_field("vm", self.vm.0)
            .var_size_bytes(&self.payload) // TODO how can we specify a field name like "payload"
            .finalize()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenesisTransaction {
    pub chain_variables: ChainVariables,
}

impl Committable for GenesisTransaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("GenesisTransaction")
            .field("chain_variables", self.chain_variables.commit())
            .finalize()
    }
}

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainVariables {
    /// The version of the protocol this chain is currently using.
    ///
    /// The protocol version can be changed by committing an update transaction.
    // TODO
    // pub protocol_version: (u16, u16, u16),

    /// A unique identifier for this chain, to prevent cross-chain replay attacks.
    ///
    /// The chain ID is set at genesis and never changes.
    pub chain_id: u16,

    // TODO: MA: this is currently not used anywhere.
    /// Committee size
    pub committee_size: u64,
}

impl Default for ChainVariables {
    fn default() -> Self {
        Self::new(
            35353, // Arbitrarily chosen.
            3,     // Arbitrarily chosen.
        )
    }
}

impl ChainVariables {
    pub fn new(chain_id: u16, committee_size: u64) -> Self {
        Self {
            chain_id,
            committee_size,
        }
    }
}

impl Committable for ChainVariables {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("ChainVariables")
            .u64_field("chain_id", self.chain_id as u64)
            .u64_field("committee_size", self.committee_size)
            .finalize()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// A transaction tht can be either a CAP transaction or a collect reward transaction
pub enum SequencerTransaction {
    Genesis(GenesisTransaction),
    Wrapped(Transaction),
}

impl Committable for SequencerTransaction {
    fn commit(&self) -> Commitment<Self> {
        let bytes = bincode::serialize(self).unwrap(); // TODO not safe unwrap?
        commit::RawCommitmentBuilder::new("SequencerTransaction")
            .var_size_bytes(&bytes)
            .finalize()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hotshot::{
        traits::implementations::{MasterMap, MemoryNetwork},
        types::HotShotHandle,
        HotShot, HotShotInitializer,
    };
    use hotshot_types::{traits::metrics::NoMetrics, ExecutionType, HotShotConfig};
    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;
    use std::time::Duration;

    #[ignore]
    #[async_std::test]
    async fn test_skeleton_instantiation() -> Result<(), ()> {
        // The minimal number of nodes is 4
        let num_nodes = 4usize;

        // Generate keys for the nodes.
        let nodes_key_pairs = (0..num_nodes)
            .map(|_| SignatureSchemeType::key_gen(&(), &mut thread_rng()).unwrap())
            .collect::<Vec<_>>();

        // Convert public keys to JfPubKey
        let nodes_pub_keys = nodes_key_pairs
            .iter()
            .map(|(_sign_key, ver_key)| JfPubKey::from_native(ver_key.clone()))
            .collect::<Vec<_>>();

        let mut handles = vec![];

        // Create HotShot instances.
        for (node_id, (sign_key, ver_key)) in nodes_key_pairs.iter().enumerate() {
            // Create public and private keys for the node.
            let public_key = JfPubKey::from_native(ver_key.clone());

            let config: HotShotConfig<_, _> = HotShotConfig {
                execution_type: ExecutionType::Continuous,
                total_nodes: num_nodes.try_into().unwrap(),
                min_transactions: 0,
                max_transactions: 1usize.try_into().unwrap(),
                known_nodes: nodes_pub_keys.clone(),
                next_view_timeout: Duration::from_secs(60).as_millis() as u64,
                timeout_ratio: (10, 11),
                round_start_delay: Duration::from_millis(1).as_millis() as u64,
                start_delay: Duration::from_millis(1).as_millis() as u64,
                num_bootstrap: 1usize,
                propose_min_round_time: Duration::from_secs(1),
                propose_max_round_time: Duration::from_secs(30),
                election_config: Some(StaticElectionConfig {}),
            };

            let network = MemoryNetwork::<SeqTypes>::new(
                public_key.clone(),
                NoMetrics::new(),
                MasterMap::new(),
                None,
            );
            let storage = MemoryStorage::<SeqTypes>::new();
            let election = StaticCommittee::<SeqTypes>::new(nodes_pub_keys.clone());
            let genesis_block = Block::genesis(Default::default());
            let initializer = HotShotInitializer::<SeqTypes>::from_genesis(genesis_block).unwrap();
            let metrics = NoMetrics::new();

            let handle: HotShotHandle<SeqTypes, Node> = HotShot::init(
                public_key,
                (sign_key.clone(), ver_key.clone()),
                node_id as u64,
                config,
                network,
                storage,
                election,
                initializer,
                metrics,
            )
            .await
            .unwrap();

            handles.push(handle);
        }

        for handle in handles.iter() {
            handle.start().await;
        }
        println!("Started");

        let event = handles[0].next_event().await;
        println!("Event: {:?}", event);

        let txn = ApplicationTransaction(vec![1, 2, 3]);

        handles[0]
            .submit_transaction(SequencerTransaction::Wrapped(Transaction {
                vm: TestVm::id(),
                payload: bincode::serialize(&txn).unwrap(),
            }))
            .await
            .expect("Failed to submit transaction");

        println!("Submitted: {:?}", txn);

        let event = handles[0].next_event().await;
        println!("Event: {:?}", event);

        Ok(())
    }
}
