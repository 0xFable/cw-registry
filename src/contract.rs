#[cfg(not(feature = "library"))]
use std::collections::BTreeMap;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdResult, WasmQuery};
use cw2::set_contract_version;
use cw_asset::AssetInfo;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, AssetQueryResponse, ContractQueryResponse};
use crate::state::{ADMIN, CONTRACT_MAP, ASSET_MAP};
use cosmwasm_storage::to_length_prefixed;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-registry2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Setup the sender as the admin of the contract
    ADMIN.set(deps, Some(info.sender.clone()))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

// Routers

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;
    match msg {
        ExecuteMsg::SetAdmin { admin } => Ok(ADMIN.execute_update_admin(
            deps,
            info,
            admin.map(|admin| api.addr_validate(&admin)).transpose()?,
        )?),        
        ExecuteMsg::UpdateContractAddresses { to_add, to_remove } => {
            update_contract_addresses(deps, info, to_add, to_remove)
        }
        ExecuteMsg::UpdateAssetAddresses { to_add, to_remove } => {
            update_asset_addresses(deps, info, to_add, to_remove)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryAssets { names } => query_assets(deps, env, &names),
        QueryMsg::QueryContracts { names } => query_contracts(deps, env, &names),
    }
}

// Query and Execute Handlers


/// Adds, updates or removes provided addresses.
pub fn update_contract_addresses(
    deps: DepsMut,
    msg_info: MessageInfo,
    to_add: Vec<(String, String)>,
    to_remove: Vec<String>,
) -> Result<Response, ContractError> {
    // Only Admin can call this method
    ADMIN.assert_admin(deps.as_ref(), &msg_info.sender)?;
    // Handle addresses to_add or update
    for (name, new_address) in to_add.into_iter() {
        let addr = deps.as_ref().api.addr_validate(&new_address)?;
        // Update function for new or existing keys
        let insert = |_| -> StdResult<Addr> { Ok(addr) };
        CONTRACT_MAP.update(deps.storage, name.as_str(), insert)?;
    }
    // Handle addresses for deletion
    for name in to_remove {
        CONTRACT_MAP.remove(deps.storage, name.as_str());
    }

    Ok(Response::new().add_attribute("action", "updated contract addresses"))
}

/// Adds, updates or removes provided addresses.
pub fn update_asset_addresses(
    deps: DepsMut,
    msg_info: MessageInfo,
    to_add: Vec<(String, AssetInfo)>,
    to_remove: Vec<String>,
) -> Result<Response, ContractError> {
    // Only Admin can call this method
    ADMIN.assert_admin(deps.as_ref(), &msg_info.sender)?;
    // Handle addresses to_add or update
    for (name, new_address) in to_add.into_iter() {
        // Update function for new or existing keys
        let insert = |_| -> StdResult<AssetInfo> { Ok(new_address) };
        ASSET_MAP.update(deps.storage, name.as_str(), insert)?;
    }
    // Handle addresses for deletion
    for name in to_remove {
        ASSET_MAP.remove(deps.storage, name.as_str());
    }

    Ok(Response::new().add_attribute("action", "updated asset addresses"))
}

/// Query asset infos from the Registry asset_map.
pub fn query_assets(
    deps: Deps,
    env: Env,
    asset_names: &[String],
) -> StdResult<Binary> {
    let mut assets: BTreeMap<String, AssetInfo> = BTreeMap::new();

    for asset in asset_names.iter() {
        let result = deps
            .querier
            .query::<AssetInfo>(&QueryRequest::Wasm(WasmQuery::Raw {
                contract_addr: env.contract.address.to_string(),
                // query assets map
                key: Binary::from(concat(&to_length_prefixed(b"assets"), asset.as_bytes())),
            }))?;
        assets.insert(asset.clone(), result);
    }
    let vector = assets.into_iter().map(|(v, k)| (v, k)).collect();
    to_binary(&AssetQueryResponse { assets: vector })
}

/// Query contract addresses from the Registry contract_map.
pub fn query_contracts(
    deps: Deps,
    env: Env,
    contract_names: &[String],
) -> StdResult<Binary> {
    let mut contracts: BTreeMap<String, Addr> = BTreeMap::new();

    // Query over
    for contract in contract_names.iter() {
        let result: Addr = deps
            .querier
            .query::<Addr>(&QueryRequest::Wasm(WasmQuery::Raw {
                contract_addr: env.contract.address.to_string(),
                key: Binary::from(concat(
                    // Query contracts map
                    &to_length_prefixed(b"contracts"),
                    contract.as_bytes(),
                )),
            }))?;

        contracts.insert(contract.clone(), result);
    }
    let vector = contracts
    .into_iter()
    .map(|(v, k)| (v, k.to_string()))
    .collect();
    to_binary(&ContractQueryResponse { contracts: vector })
}

#[inline]
fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = namespace.to_vec();
    k.extend_from_slice(key);
    k
}

