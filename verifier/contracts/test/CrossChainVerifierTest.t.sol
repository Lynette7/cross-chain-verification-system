// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {CrossChainVerifier} from "../src/CrossChainVerifier.sol";
import {SP1VerifierGateway} from "../lib/sp1-contracts/contracts/src/SP1VerifierGateway.sol";

// Struct to load the proof fixture JSON
struct SP1ProofFixtureJson {
    bytes32 messageHash;
    uint32 sourceChainId;
    uint32 destinationChainId;
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}

contract CrossChainVerifierTest is Test {
    using stdJson for string;

    address verifier;
    CrossChainVerifier public crossChainVerifier;

    // Load the SP1 proof fixture JSON from the project fixtures
    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/fixtures/cross-chain-fixture.json");
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    // Setup the verifier contract and the cross-chain verifier contract
    function setUp() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        verifier = address(new SP1VerifierGateway(address(1)));
        crossChainVerifier = new CrossChainVerifier(verifier, fixture.vkey);
    }

    // Test the valid proof for cross-chain message processing
    function test_ValidCrossChainMessageProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Mock the verifier contract call to always return true (valid proof)
        vm.mockCall(verifier, abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector), abi.encode(true));

        // Verify the proof and decode the public values
        (bytes32 messageHash, uint32 sourceChainId, uint32 destinationChainId) = crossChainVerifier.verifyCrossChainMessageProof(fixture.publicValues, fixture.proof);

        // Assert that the decoded values match the expected fixture values
        assert(messageHash == fixture.messageHash);
        assert(sourceChainId == fixture.sourceChainId);
        assert(destinationChainId == fixture.destinationChainId);
    }

    // Test the failure case for an invalid proof
    function testFail_InvalidCrossChainMessageProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof
        bytes memory fakeProof = new bytes(fixture.proof.length);

        // Verify the fake proof (this should fail)
        crossChainVerifier.verifyCrossChainMessageProof(fixture.publicValues, fakeProof);
    }
}