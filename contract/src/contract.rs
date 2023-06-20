use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config_read, config_write, Config};

// version info for migration info
const CONTRACT_NAME: &str = "bulletproofs";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const TRANSCRIPT_BYTES: &[u8] = b"Oraichain.RangeProof.Age";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    config_write(
        deps.storage,
        &Config {
            contract: CONTRACT_NAME.to_string(),
            version: CONTRACT_VERSION.to_string(),
            owner: info.sender,
            n: msg.n,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetOwner { new_owner } => set_owner(deps, info, new_owner),
    }
}

pub fn set_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: Addr,
) -> Result<Response, ContractError> {
    let mut config = config_read(deps.storage)?;
    if info.sender == config.owner {
        config.owner = new_owner;
        config_write(deps.storage, &config)?;
        Ok(Response::default())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerifyProof {
            commitment,
            proof_raw,
            challenge,
        } => to_binary(&verify_proof(deps, &commitment, &proof_raw, &challenge)?),
    }
}

#[allow(non_snake_case)]
pub fn verify_proof(
    deps: Deps,
    commitment: &[u8],
    proof_raw: &[u8],
    challenge: &[u8],
) -> StdResult<bool> {
    let config = config_read(deps.storage)?;
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 8);
    let range_proof = RangeProof::from_bytes(proof_raw)
        .map_err(|_| StdError::generic_err("Range proof is not correct"))?;
    let value_commitment = CompressedRistretto::from_slice(commitment);
    let mut transcript = Transcript::new(TRANSCRIPT_BYTES);
    Ok(range_proof
        .verify_single_with_challenge(
            &bp_gens,
            &pc_gens,
            &mut transcript,
            &value_commitment,
            &Scalar::hash_from_bytes::<sha3::Sha3_512>(challenge),
            config.n as usize,
        )
        .is_ok())
}
