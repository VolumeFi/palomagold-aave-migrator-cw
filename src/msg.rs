use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CustomMsg, Uint256};

use crate::state::ChainSetting;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterChain {
        chain_id: String,
        chain_setting: ChainSetting,
    },
    Release {
        chain_id: String,
        recipient: String,
        amount: Uint256,
        nonce: String,
    },

    // Set Paloma address of a chain
    SetPaloma {
        chain_id: String,
    },
    // Update Refund Wallet
    UpdateRefundWallet {
        chain_id: String,
        new_refund_wallet: String,
    },
    // Update Gas Fee
    UpdateGasFee {
        chain_id: String,
        new_gas_fee: Uint256,
    },
    // Update Service Fee Collector
    UpdateServiceFeeCollector {
        chain_id: String,
        new_service_fee_collector: String,
    },
    // Update Service Fee
    UpdateServiceFee {
        chain_id: String,
        new_service_fee: Uint256,
    },
}


#[cw_serde]
pub enum PalomaMsg {
    /// Message struct for cross-chain calls.
    SchedulerMsg { execute_job: ExecuteJob },
}

#[cw_serde]
pub struct ExecuteJob {
    pub job_id: String,
    pub payload: Binary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

impl CustomMsg for PalomaMsg {}