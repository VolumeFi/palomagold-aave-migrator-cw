use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CustomMsg, Uint128, Uint256};

use crate::state::ChainSetting;

#[cw_serde]
pub struct InstantiateMsg {
    pub palomagold_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterChain {
        chain_id: String,
        chain_setting: ChainSetting,
    },
    SendPalomaGold {
        chain_id: String,
        recipient: String,
        amount: Uint128,
    },
    Release {
        chain_id: String,
        recipient: String,
        amount: Uint256,
        nonce: Uint256,
    },
    CancelTx {
        transaction_id: u64,
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
    SchedulerMsg {
        execute_job: ExecuteJob,
    },
    SkywayMsg {
        send_tx: Option<SendTx>,
        cancel_tx: Option<CancelTx>,
    },
}

#[cw_serde]
pub struct ExecuteJob {
    pub job_id: String,
    pub payload: Binary,
}

#[cw_serde]
pub struct SendTx {
    pub remote_chain_destination_address: String,
    pub amount: String,
    pub chain_reference_id: String,
}

#[cw_serde]
pub struct CancelTx {
    pub transaction_id: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalanceResponse)]
    PalomagoldBalance {},
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

impl CustomMsg for PalomaMsg {}
