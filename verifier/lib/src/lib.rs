use alloy_sol_types::sol;
use sha2::{Digest, Sha256};

sol! {
    struct CrossChainMessageStruct {
        bytes32 message_hash;
        uint32 source_chain_id;
        uint32 destination_chain_id;
    }
}

/// Compute a hash of the input message using SHA256.
pub fn hash_message(message: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(message);
    hasher.finalize().into()
}
