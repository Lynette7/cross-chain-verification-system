#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod cross_chain_messaging {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    // The main contract structure.
    #[ink(storage)]
    pub struct CrossChainMessaging {
        /// Keeps track of the latest nonce for incoming messages.
        incoming_nonce: u64,
        /// Keeps track of the latest nonce for outgoing messages.
        outgoing_nonce: u64,
        /// Mapping to store messages received from Ethereum.
        messages: Mapping<u64, String>,
    }

    /// Structure for proof data.
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ProofData {
        // Placeholder for actual proof structure (depends on SP1 proof format)
        pub proof: Vec<u8>,
        pub message_hash: [u8; 32],
    }

    /// Structure for outgoing messages.
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct OutgoingMessage {
        pub data: String,
        pub nonce: u64,
    }

    impl CrossChainMessaging {
        // Constructor to initialize the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                incoming_nonce: 0,
                outgoing_nonce: 0,
                messages: Mapping::default(),
            }
        }

        /// Store an incoming message from Ethereum.
        #[ink(message)]
        pub fn store_message(&mut self, data: String, proof: ProofData) -> bool {
            // Verify the proof using SP1 (stubbed for now)
            if !self.verify_proof(proof, &data) {
                return false;
            }

            // Increment the incoming nonce
            self.incoming_nonce += 1;

            // Store the message
            self.messages.insert(self.incoming_nonce, &data);

            // Emit an event for the stored message
            self.env().emit_event(MessageStored {
                nonce: self.incoming_nonce,
                data: data.clone(),
            });

            true
        }

        // TODO: proof verification for an incoming message
        #[ink(message)]
        pub fn verify_proof<'a>(&self, _proof: ProofData, _data: &'a String) -> bool {
            // TODO
            true
        }

        /// Retrieve a stored message by its nonce.
        #[ink(message)]
        pub fn get_message(&self, nonce: u64) -> Option<String> {
            self.messages.get(nonce)
        }

        /// Prepare an outgoing message to Ethereum.
        #[ink(message)]
        pub fn prepare_outgoing_message(&mut self, data: String) -> OutgoingMessage {
            // Increment the outgoing nonce
            self.outgoing_nonce += 1;

            // Construct the outgoing message
            let outgoing_message = OutgoingMessage {
                data: data.clone(),
                nonce: self.outgoing_nonce,
            };

            // Emit an event for the outgoing message
            self.env().emit_event(MessagePrepared {
                nonce: self.outgoing_nonce,
                data,
            });

            outgoing_message
        }

        /// Get the current incoming nonce.
        #[ink(message)]
        pub fn get_incoming_nonce(&self) -> u64 {
            self.incoming_nonce
        }

        /// Get the current outgoing nonce.
        #[ink(message)]
        pub fn get_outgoing_nonce(&self) -> u64 {
            self.outgoing_nonce
        }
    }

    /// Event emitted when a message is stored.
    #[ink(event)]
    pub struct MessageStored {
        #[ink(topic)]
        nonce: u64,
        data: String,
    }

    /// Event emitted when an outgoing message is prepared.
    #[ink(event)]
    pub struct MessagePrepared {
        #[ink(topic)]
        nonce: u64,
        data: String,
    }
}