use anyhow::anyhow;
use ethers::prelude::EIP1186ProofResponse;
use ethers::{types::U256, utils::keccak256};
use helios::consensus::ConsensusRpc;
use helios::dwallet::encode_account;
use helios::dwallet::light_client::ProofRequestParameters;
use helios::execution::{get_message_storage_slot, verify_proof};
use helios::types::Address;

pub fn try_verify_proof(
    proof: EIP1186ProofResponse,
    contract_address: &Address,
    proof_params: ProofRequestParameters,
    state_root: Vec<u8>,
) -> anyhow::Result<bool> {
    const TRUE_VALUE: u8 = 1;

    let message_map_index = get_message_storage_slot(
        proof_params.message.clone(),
        proof_params.dwallet_id.clone(),
        proof_params.data_slot,
    )
    .map_err(|e| anyhow!(e))?;

    let account_path = keccak256(contract_address.as_bytes()).to_vec();
    let account_encoded = encode_account(&proof);

    // Verify the account proof against the state root.
    let is_valid_account_proof = verify_proof(
        &proof.clone().account_proof,
        &state_root,
        &account_path,
        &account_encoded,
    );

    if !is_valid_account_proof {
        return Ok(false);
    }

    // Get only the proof that matches the message_map_index.
    let msg_storage_proof = proof
        .storage_proof
        .iter()
        .find(|p| p.key == U256::from(message_map_index.as_bytes()))
        .ok_or_else(|| anyhow!("No storage proof found for key `message_map_index`."))?;
    // Cast the storage key to a 32-byte array, and hash using keccak256 algorithm.
    let mut msg_storage_proof_key_bytes = [0u8; 32];
    msg_storage_proof
        .key
        .to_big_endian(&mut msg_storage_proof_key_bytes);

    let storage_key_hash = keccak256(msg_storage_proof_key_bytes);
    let storage_value = [TRUE_VALUE].to_vec();

    Ok(verify_proof(
        &msg_storage_proof.clone().proof,
        &proof.storage_hash.as_bytes().to_vec(),
        &storage_key_hash.to_vec(),
        &storage_value,
    ))
}
