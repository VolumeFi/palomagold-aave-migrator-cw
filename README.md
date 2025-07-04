# Palomagold AAVE Migrator CosmWasm

## Overview

This CosmWasm smart contract facilitates cross-chain operations for the Palomagold token, including bridging, chain registration, and administrative updates. It is designed for secure, auditable, and upgradable deployments.

---

## State and Data Structures

### State
- **State**: Stores the contract owner and the Palomagold token denomination.
- **ChainSetting**: Stores per-chain configuration, including the job ID for cross-chain operations.
- **Storage Keys**:
  - `STATE`: Singleton for contract state.
  - `CHAIN_SETTINGS`: Map of chain IDs to their settings.
  - `WITHDRAW_TIMESTAMP`: Map of (chain_id, nonce) to withdrawal timestamps (prevents replay attacks).

### Error Types
- `Unauthorized`: The sender is not the contract owner.
- `Pending`: An operation is pending (e.g., release attempted too soon).
- `Std`: Standard CosmWasm error.

---

## Entry Points and Function Documentation

### 1. `instantiate`
Initializes the contract with the owner and Palomagold denomination.

**Signature:**
```rust
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<Response, ContractError>
```
**Parameters:**
- `palomagold_denom` (String): The denomination of the Palomagold token.

**Example:**
```json
{
  "palomagold_denom": "palomagold"
}
```

---

### 2. `migrate`
Upgrades the contract to a new version.

**Signature:**
```rust
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError>
```
**Parameters:** None (empty message).

**Example:**
```json
{}
```

---

### 3. `execute`
Handles all executable messages. Each variant is described below.

#### a. `RegisterChain`
Registers a new chain and its settings. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::RegisterChain { chain_id, chain_setting }
```
**Parameters:**
- `chain_id` (String): The chain identifier.
- `chain_setting` (ChainSetting): Contains `job_id` (String).

**Example:**
```json
{
  "register_chain": {
    "chain_id": "eth-mainnet",
    "chain_setting": { "job_id": "job123" }
  }
}
```

#### b. `SendPalomaGold`
Bridges Palomagold tokens to a recipient on another chain. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::SendPalomaGold { chain_id, recipient, amount }
```
**Parameters:**
- `chain_id` (String)
- `recipient` (String)
- `amount` (Uint128)

**Example:**
```json
{
  "send_paloma_gold": {
    "chain_id": "eth-mainnet",
    "recipient": "0xabc...",
    "amount": "1000000"
  }
}
```

#### c. `Release`
Releases funds to a recipient on another chain after a delay. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::Release { chain_id, recipient, amount, nonce }
```
**Parameters:**
- `chain_id` (String)
- `recipient` (String)
- `amount` (Uint256)
- `nonce` (Uint256)

**Example:**
```json
{
  "release": {
    "chain_id": "eth-mainnet",
    "recipient": "0xabc...",
    "amount": "1000000",
    "nonce": "1"
  }
}
```

#### d. `CancelTx`
Cancels a pending cross-chain transaction. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::CancelTx { transaction_id }
```
**Parameters:**
- `transaction_id` (u64)

**Example:**
```json
{
  "cancel_tx": { "transaction_id": 42 }
}
```

#### e. `SetPaloma`
Sets the Paloma address for a chain. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::SetPaloma { chain_id }
```
**Parameters:**
- `chain_id` (String)

**Example:**
```json
{
  "set_paloma": { "chain_id": "eth-mainnet" }
}
```

#### f. `UpdateRefundWallet`
Updates the refund wallet address for a chain. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::UpdateRefundWallet { chain_id, new_refund_wallet }
```
**Parameters:**
- `chain_id` (String)
- `new_refund_wallet` (String)

**Example:**
```json
{
  "update_refund_wallet": {
    "chain_id": "eth-mainnet",
    "new_refund_wallet": "0xdef..."
  }
}
```

#### g. `UpdateGasFee`
Updates the gas fee for a chain. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::UpdateGasFee { chain_id, new_gas_fee }
```
**Parameters:**
- `chain_id` (String)
- `new_gas_fee` (Uint256)

**Example:**
```json
{
  "update_gas_fee": {
    "chain_id": "eth-mainnet",
    "new_gas_fee": "50000"
  }
}
```

#### h. `UpdateServiceFeeCollector`
Updates the service fee collector address for a chain. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::UpdateServiceFeeCollector { chain_id, new_service_fee_collector }
```
**Parameters:**
- `chain_id` (String)
- `new_service_fee_collector` (String)

**Example:**
```json
{
  "update_service_fee_collector": {
    "chain_id": "eth-mainnet",
    "new_service_fee_collector": "0x123..."
  }
}
```

#### i. `UpdateServiceFee`
Updates the service fee for a chain. Only the owner can call this.

**Signature:**
```rust
ExecuteMsg::UpdateServiceFee { chain_id, new_service_fee }
```
**Parameters:**
- `chain_id` (String)
- `new_service_fee` (Uint256)

**Example:**
```json
{
  "update_service_fee": {
    "chain_id": "eth-mainnet",
    "new_service_fee": "1000"
  }
}
```

---

### 4. `query`
Handles all query messages.

#### a. `PalomagoldBalance`
Returns the Palomagold token balance held by the contract.

**Signature:**
```rust
QueryMsg::PalomagoldBalance {}
```
**Returns:**
- `balance` (Uint128): The contract's Palomagold balance.

**Example:**
```json
{
  "palomagold_balance": {}
}
```
**Response:**
```json
{
  "balance": "1000000"
}
```

---

## Internal Logic and Security Considerations
- **Authorization:** All state-changing operations are restricted to the contract owner.
- **Replay Protection:** The `WITHDRAW_TIMESTAMP` map ensures that releases cannot be replayed within a short window.
- **Cross-Chain Safety:** All cross-chain operations are routed through job IDs and payloads, ensuring traceability and auditability.
- **Error Handling:** Custom errors are used for unauthorized access and pending operations.

---

## Code Generation
The `src/bin/schema.rs` file generates JSON schema for all messages:
```rust
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
```

---

## Contact
For questions or audits, contact the maintainers or open an issue in this repository.