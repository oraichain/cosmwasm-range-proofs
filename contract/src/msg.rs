use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary};

#[cw_serde]
pub struct InstantiateMsg {
    pub n: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetOwner { new_owner: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    VerifyProof {
        commitment: Binary,
        proof_raw: Binary,
        /// unique
        challenge: Binary,
    },
}
