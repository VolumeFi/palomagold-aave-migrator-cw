#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PalomaMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:palomagold-aave-migrator-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        palomagold_denom: msg.palomagold_denom,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    match msg {
        ExecuteMsg::RegisterChain {
            chain_id,
            chain_setting,
        } => execute::register_chain(deps, info, chain_id, chain_setting),
        ExecuteMsg::SendPalomaGold {
            chain_id,
            recipient,
            amount,
        } => execute::send_paloma_gold(deps, info, chain_id, recipient, amount),
        ExecuteMsg::Release {
            chain_id,
            recipient,
            amount,
            nonce,
        } => execute::release(deps, env, info, chain_id, recipient, amount, nonce),
        ExecuteMsg::SetPaloma { chain_id } => execute::set_paloma(deps, info, chain_id),
        ExecuteMsg::UpdateRefundWallet {
            chain_id,
            new_refund_wallet,
        } => execute::update_refund_wallet(deps, info, chain_id, new_refund_wallet),
        ExecuteMsg::UpdateGasFee {
            chain_id,
            new_gas_fee,
        } => execute::update_gas_fee(deps, info, chain_id, new_gas_fee),
        ExecuteMsg::UpdateServiceFeeCollector {
            chain_id,
            new_service_fee_collector,
        } => execute::update_service_fee_collector(deps, info, chain_id, new_service_fee_collector),
        ExecuteMsg::UpdateServiceFee {
            chain_id,
            new_service_fee,
        } => execute::update_service_fee(deps, info, chain_id, new_service_fee),
    }
}

pub mod execute {
    use cosmwasm_std::{Coin, CosmosMsg, Uint128, Uint256};
    use ethabi::{Address, Contract, Function, Param, ParamType, StateMutability, Token, Uint};
    use std::collections::BTreeMap;
    use std::str::FromStr;

    use super::*;
    use crate::{
        msg::{ExecuteJob, SendTx},
        state::{ChainSetting, CHAIN_SETTINGS, WITHDRAW_TIMESTAMP},
    };

    pub fn register_chain(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
        chain_setting: ChainSetting,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        CHAIN_SETTINGS.save(deps.storage, chain_id.clone(), &chain_setting)?;
        Ok(Response::new()
            .add_attribute("action", "register_chain")
            .add_attribute("chain_id", chain_id))
    }

    pub fn send_paloma_gold(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
        recipient: String,
        amount: Uint128,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        // Implement the logic for sending Paloma Gold
        let state = STATE.load(deps.storage)?;
        assert!(info.sender == state.owner, "Unauthorized");
        let coin_to_bridge: Coin = Coin {
            denom: state.palomagold_denom.clone(),
            amount,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SkywayMsg {
                send_tx: SendTx {
                    remote_chain_destination_address: recipient.clone(),
                    amount: coin_to_bridge.to_string(),
                    chain_reference_id: chain_id.clone(),
                },
            }))
            .add_attribute("action", "send_paloma_gold"))
    }

    pub fn release(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        chain_id: String,
        recipient: String,
        amount: Uint256,
        nonce: Uint256,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        // Implement the logic for releasing funds
        let state = STATE.load(deps.storage)?;
        assert!(info.sender == state.owner, "Unauthorized");

        if let Some(timestamp) =
            WITHDRAW_TIMESTAMP.may_load(deps.storage, (chain_id.clone(), nonce.to_string()))?
        {
            if timestamp.plus_seconds(60).gt(&env.block.time) {
                // If the timestamp is not older than 60 seconds, return an error
                return Err(ContractError::Pending {});
            }
        }

        let recipient_address: Address = Address::from_str(recipient.as_str()).unwrap();
        let amount: Uint = Uint::from_big_endian(&amount.to_be_bytes());
        let nonce: Uint = Uint::from_big_endian(&nonce.to_be_bytes());
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "release".to_string(),
                vec![Function {
                    name: "release".to_string(),
                    inputs: vec![
                        Param {
                            name: "recipient".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        },
                        Param {
                            name: "amount".to_string(),
                            kind: ParamType::Uint(256),
                            internal_type: None,
                        },
                        Param {
                            name: "nonce".to_string(),
                            kind: ParamType::Uint(256),
                            internal_type: None,
                        },
                    ],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };

        WITHDRAW_TIMESTAMP.save(
            deps.storage,
            (chain_id.clone(), nonce.to_string()),
            &env.block.time,
        )?;

        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                execute_job: ExecuteJob {
                    job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                    payload: Binary::new(
                        contract
                            .function("release")
                            .unwrap()
                            .encode_input(&[
                                Token::Address(recipient_address),
                                Token::Uint(amount),
                                Token::Uint(nonce),
                            ])
                            .unwrap(),
                    ),
                },
            }))
            .add_attribute("action", "release"))
    }

    pub fn set_paloma(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        // ACTION: Implement SetPaloma
        let state = STATE.load(deps.storage)?;
        assert!(info.sender == state.owner, "Unauthorized");

        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "set_paloma".to_string(),
                vec![Function {
                    name: "set_paloma".to_string(),
                    inputs: vec![],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                execute_job: ExecuteJob {
                    job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                    payload: Binary::new(
                        contract
                            .function("set_paloma")
                            .unwrap()
                            .encode_input(&[])
                            .unwrap(),
                    ),
                },
            }))
            .add_attribute("action", "set_paloma"))
    }

    pub fn update_refund_wallet(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
        new_refund_wallet: String,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        assert!(state.owner == info.sender, "Unauthorized");
        let new_refund_wallet_address: Address =
            Address::from_str(new_refund_wallet.as_str()).unwrap();
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "update_refund_wallet".to_string(),
                vec![Function {
                    name: "update_refund_wallet".to_string(),
                    inputs: vec![Param {
                        name: "new_refund_wallet".to_string(),
                        kind: ParamType::Address,
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                execute_job: ExecuteJob {
                    job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                    payload: Binary::new(
                        contract
                            .function("update_refund_wallet")
                            .unwrap()
                            .encode_input(&[Token::Address(new_refund_wallet_address)])
                            .unwrap(),
                    ),
                },
            }))
            .add_attribute("action", "update_refund_wallet"))
    }

    pub fn update_gas_fee(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
        new_gas_fee: Uint256,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        // ACTION: Implement UpdateGasFee
        let state = STATE.load(deps.storage)?;
        assert!(info.sender == state.owner, "Unauthorized");
        let new_gas_fee: Uint = Uint::from_big_endian(&new_gas_fee.to_be_bytes());
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "update_gas_fee".to_string(),
                vec![Function {
                    name: "update_gas_fee".to_string(),
                    inputs: vec![Param {
                        name: "new_gas_fee".to_string(),
                        kind: ParamType::Uint(256),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                execute_job: ExecuteJob {
                    job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                    payload: Binary::new(
                        contract
                            .function("update_gas_fee")
                            .unwrap()
                            .encode_input(&[Token::Uint(new_gas_fee)])
                            .unwrap(),
                    ),
                },
            }))
            .add_attribute("action", "update_gas_fee"))
    }

    pub fn update_service_fee_collector(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
        new_service_fee_collector: String,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        // ACTION: Implement UpdateServiceFeeCollector
        let state = STATE.load(deps.storage)?;
        assert!(info.sender == state.owner, "Unauthorized");
        let new_service_fee_collector: Address =
            Address::from_str(new_service_fee_collector.as_str()).unwrap();
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "update_service_fee_collector".to_string(),
                vec![Function {
                    name: "update_service_fee_collector".to_string(),
                    inputs: vec![Param {
                        name: "new_service_fee_collector".to_string(),
                        kind: ParamType::Address,
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                execute_job: ExecuteJob {
                    job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                    payload: Binary::new(
                        contract
                            .function("update_service_fee_collector")
                            .unwrap()
                            .encode_input(&[Token::Address(new_service_fee_collector)])
                            .unwrap(),
                    ),
                },
            }))
            .add_attribute("action", "update_service_fee_collector"))
    }

    pub fn update_service_fee(
        deps: DepsMut,
        info: MessageInfo,
        chain_id: String,
        new_service_fee: Uint256,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        // ACTION: Implement UpdateServiceFee
        let state = STATE.load(deps.storage)?;
        assert!(info.sender == state.owner, "Unauthorized");
        let new_service_fee: Uint = Uint::from_big_endian(&new_service_fee.to_be_bytes());
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "update_service_fee".to_string(),
                vec![Function {
                    name: "update_service_fee".to_string(),
                    inputs: vec![Param {
                        name: "new_service_fee".to_string(),
                        kind: ParamType::Uint(256),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                execute_job: ExecuteJob {
                    job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                    payload: Binary::new(
                        contract
                            .function("update_service_fee")
                            .unwrap()
                            .encode_input(&[Token::Uint(new_service_fee)])
                            .unwrap(),
                    ),
                },
            }))
            .add_attribute("action", "update_service_fee"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PalomagoldBalance {} => query::palomagold_balance(deps, env),
    }
}

pub mod query {
    use cosmwasm_std::to_json_binary;

    use super::*;
    use crate::msg::BalanceResponse;

    pub fn palomagold_balance(deps: Deps, env: Env) -> StdResult<Binary> {
        to_json_binary(&BalanceResponse {
            balance: deps
                .querier
                .query_balance(
                    env.contract.address,
                    STATE.load(deps.storage)?.palomagold_denom,
                )?
                .amount,
        })
    }
}

#[cfg(test)]
mod tests {}
