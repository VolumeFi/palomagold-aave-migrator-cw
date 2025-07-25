use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub palomagold_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ChainSetting {
    pub job_id: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const CHAIN_SETTINGS: Map<String, ChainSetting> = Map::new("chain_settings");
pub const WITHDRAW_TIMESTAMP: Map<(String, String), Timestamp> = Map::new("withdraw_timestamp");
