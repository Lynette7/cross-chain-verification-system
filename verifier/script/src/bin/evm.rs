//! An end-to-end example of using the SP1 SDK to generate a proof for cross-chain message processing,
//! with Solidity-compatible public values and proofs for on-chain verification.

use alloy_sol_types::SolType;
use sp1_sdk::HashableKey;
use clap::{Parser, ValueEnum};
use cross_chain_lib::{CrossChainMessageStruct, hash_message};
use serde::{Deserialize, Serialize};
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const CROSS_CHAIN_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the EVM command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct EVMArgs {
    #[clap(long, default_value = "20")]
    n: u32, // Not used, but included for consistency

    #[clap(long, value_enum, default_value = "groth16")]
    system: ProofSystem,

    #[clap(long)]
    message: String,

    #[clap(long)]
    source_chain_id: u32,

    #[clap(long)]
    destination_chain_id: u32,
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

/// Fixture for testing cross-chain proofs with Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1CrossChainProofFixture {
    message_hash: String,
    source_chain_id: u32,
    destination_chain_id: u32,
    vkey: String,
    public_values: String,
    proof: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = EVMArgs::parse();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the program.
    let (pk, vk) = client.setup(CROSS_CHAIN_ELF);

    // Setup the inputs.
    let input_message = args.message.as_bytes().to_vec();
    let source_chain_id = args.source_chain_id;
    let destination_chain_id = args.destination_chain_id;

    // Create stdin to pass inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&input_message);
    stdin.write(&source_chain_id);
    stdin.write(&destination_chain_id);

    println!("Message: {}", args.message);
    println!("Source Chain ID: {}", source_chain_id);
    println!("Destination Chain ID: {}", destination_chain_id);
    println!("Proof System: {:?}", args.system);

    // Generate the proof based on the selected proof system.
    let proof = match args.system {
        ProofSystem::Plonk => client.prove(&pk, stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, stdin).groth16().run(),
    }
    .expect("failed to generate proof");

    // Create the fixture for the proof.
    create_proof_fixture(&proof, &vk, input_message, source_chain_id, destination_chain_id, args.system);
}

/// Create a fixture for the given proof.
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    input_message: Vec<u8>,
    source_chain_id: u32,
    destination_chain_id: u32,
    system: ProofSystem,
) {
    // Deserialize the public values.
    let bytes = proof.public_values.as_slice();
    let CrossChainMessageStruct { message_hash, source_chain_id, destination_chain_id } = CrossChainMessageStruct::abi_decode(bytes, false).unwrap();

    // Create the testing fixture for Solidity verification.
    let fixture = SP1CrossChainProofFixture {
        message_hash: format!("0x{:x}", message_hash),
        source_chain_id,
        destination_chain_id,
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    // Save the fixture to a file for Solidity use.
    let fixture_path = PathBuf::from(format!("proof_fixture_{:?}.json", system));
    let mut file = File::create(fixture_path).expect("Failed to create proof fixture file");
    let fixture_json = serde_json::to_string(&fixture).expect("Failed to serialize proof fixture");
    file.write_all(fixture_json.as_bytes()).expect("Failed to write fixture to file");

    println!("Proof fixture saved.");
}

