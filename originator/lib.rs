#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod cross_chain_verifier {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
    use scale::{Decode, Encode};

    #[derive(Debug, Encode, Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CrossChainMessage {
        source_chain_id: u32,
        nonce: u64,
        sender: [u8; 20],
        payload: Vec<u8>,
        signature: Vec<u8>,
    }

    #[derive(Debug, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SP1Proof {
        proof_bytes: Vec<u8>,
        public_inputs: Vec<u8>,
        public_outputs: Vec<u8>,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct CrossChainVerfier {
        owner: AccountId,
        verified_messages: Mapping<[u8;32], bool>,
        verifier_key:Vec<u8>,
        processed_nonces: Mapping<(u32, u64), bool>,
    }

    // Errors that can occur during contract execution
    #[derive(Debug, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        MessageAlreadyProcessed,
        InvalidProof,
        InvalidNonce,
        Unauthorized,
    }

    // Event emitted when a message is verfied
    #[ink(event)]
    pub struct MessageVerified {
        #[ink(topic)]
        message_hash: [u8; 32],
        source_chain_id: u32,
        nonce: u64,
        sender: [u8; 20],
    }

    impl CrossChainVerifier {
        #[ink(constructor)]
        pub fn new(initial_verifier_key: Vec<u8>) -> Self {
            ink::lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = Self::env().caller();
                contract.verifier_key = initial_verifier_key;
            })
        }

        // Verify a cross-chain message using SP1 zkvm proof
        #[ink(message)]
        pub fn verify_message(&mut self, message: CrossChainMessage, proof: SP1Proof) -> Result<bool, Error> {
            if self.processed_nonces.get((message.source_chain_id, message.nonce))
            .unwrap_or(false) {
                return Err(Error::MessageAlreadyProcessed);
            }

            let message_hash = self.calculate_message_hash(&message);

            if !self.verify_sp1_proof(&proof, &message_hash) {
                return Err(Error::InvalidProof);
            }

            self.verified_messages.insert(message_hash, &true);
            self.processed_nonces.insert((message.source_chain_id, message.nonce), &true);

            self.env().emit_event(MessageVerified {
                message_hash,
                source_chain_id: message.source_chain_id,
                nonce: message.nonce,
                sender: message.sender,
            });

            Ok(true)
        }
    }
}
