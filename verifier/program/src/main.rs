#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use cross_chain_lib::{hash_message, CrossChainMessageStruct};

pub fn main() {
    // Read the input message, source chain ID, and destination chain ID.
    let input_message = sp1_zkvm::io::read::<Vec<u8>>();
    let source_chain_id = sp1_zkvm::io::read::<u32>();
    let destination_chain_id = sp1_zkvm::io::read::<u32>();

    // Validate inputs (basic validation for now).
    if source_chain_id == destination_chain_id {
        panic!("Source chain ID and destination chain ID cannot be the same.");
    }
    if input_message.is_empty() {
        panic!("Input message cannot be empty.");
    }

    // Compute a hash of the message for tamper-proofing.
    let message_hash = hash_message(&input_message);

    // Encode the public values of the program.
    let public_values = CrossChainMessageStruct {
        // message_hash,
        message_hash: alloy_sol_types::private::FixedBytes(message_hash),
        source_chain_id,
        destination_chain_id,
    };
    let encoded_values = CrossChainMessageStruct::abi_encode(&public_values);

    // Commit to the public values. This will be included in the zkVM proof.
    sp1_zkvm::io::commit_slice(&encoded_values);
}
