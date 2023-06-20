use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_slice, to_vec, Addr, StdError, StdResult, Storage};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub contract: String,
    pub version: String,
    pub n: u8,
}

pub fn config_write(storage: &mut dyn Storage, data: &Config) -> StdResult<()> {
    Ok(storage.set(CONFIG_KEY, &to_vec(data)?))
}
pub fn config_read(storage: &dyn Storage) -> StdResult<Config> {
    match storage.get(CONFIG_KEY) {
        Some(data) => from_slice(&data),
        None => Err(StdError::generic_err("Config not found")),
    }
}

pub const CONFIG_KEY: &[u8] = b"CONFIG";
