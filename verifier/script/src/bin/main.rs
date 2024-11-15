//! An end-to-end example of using the SP1 SDK to generate a proof of a program that processes
//! a cross-chain message, verifies its integrity, and encodes public values for proof generation.

use alloy_sol_types::SolType;
use clap::Parser;
use serde_json;
use cross_chain_lib::{CrossChainMessageStruct, hash_message};
use sp1_sdk::{ProverClient, SP1Stdin};
use std::fs::File;
use std::io::Write;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const CROSS_CHAIN_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    message: String,

    #[clap(long)]
    source_chain_id: u32,

    #[clap(long)]
    destination_chain_id: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    let input_message = args.message.as_bytes().to_vec();
    let source_chain_id = args.source_chain_id;
    let destination_chain_id = args.destination_chain_id;

    stdin.write(&input_message);
    stdin.write(&source_chain_id);
    stdin.write(&destination_chain_id);

    println!("Message: {}", args.message);
    println!("Source Chain ID: {}", source_chain_id);
    println!("Destination Chain ID: {}", destination_chain_id);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(CROSS_CHAIN_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Decode and verify the output (public values)
        let decoded = CrossChainMessageStruct::abi_decode(output.as_slice(), true).unwrap();
        let CrossChainMessageStruct { message_hash, source_chain_id, destination_chain_id } = decoded;

        println!("Message Hash: {:x}", message_hash);
        println!("Source Chain ID: {}", source_chain_id);
        println!("Destination Chain ID: {}", destination_chain_id);

        // Verify that the program computed the correct hash
        let expected_hash = hash_message(&input_message);
        assert_eq!(message_hash, expected_hash);
        println!("Message hash is correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(CROSS_CHAIN_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");

        // Save the verification key and proof for Solidity integration
        let proof_file_path = "proof.json";
        let mut file = File::create(proof_file_path).expect("Failed to create proof file");
        file.write_all(&proof.bytes()).expect("Failed to write proof to file");

        let vk_file_path = "verification_key.json";
        let mut vk_file = File::create(vk_file_path).expect("Failed to create verification key file");
        // vk_file.write_all(&vk.bytes32()).expect("Failed to write verification key to file");
        let serialized_vk = serde_json::to_vec(&vk).expect("Failed to serialize verification key");
        vk_file.write_all(&serialized_vk).expect("Failed to write verification key to file");

        println!("Verification key and proof saved.");
    }
}
