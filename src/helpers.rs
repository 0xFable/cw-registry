use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use cw_asset::AssetInfo;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, CustomQuery, Querier, QueryRequest, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};
use cosmwasm_storage::to_length_prefixed;

use crate::msg::{ExecuteMsg};

/// RegistryController is a wrapper around Addr that provides helpings for controlling this contract
/// for example in another contract by importing the controller into your contract. 
/// It can also help alot with testing.
/// Considering renaming this as I have a 'controller' concept in my other packages already
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegistryController(pub Addr);

impl RegistryController {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn query_contracts<Q, T, CQ>(&self, querier: &Q, contract_names: &[String]) -> StdResult<BTreeMap<String, Addr>> 
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let mut contracts: BTreeMap<String, Addr> = BTreeMap::new();

        // Query over
        for contract in contract_names.iter() {
            let result: Addr = QuerierWrapper::<CQ>::new(querier)
                .query::<Addr>(&QueryRequest::Wasm(WasmQuery::Raw {
                    contract_addr: self.addr().into(),
                    key: Binary::from(concat(
                        // Query contracts map
                        &to_length_prefixed(b"contracts"),
                        contract.as_bytes(),
                    )),
                }))?;
    
            contracts.insert(contract.clone(), result);
        }
        Ok(contracts)
    }
    pub fn query_assets<Q, T, CQ>(&self, querier: &Q, asset_names: &[String]) -> StdResult<BTreeMap<String, AssetInfo>> 
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let mut assets: BTreeMap<String, AssetInfo> = BTreeMap::new();

        // Query over
        for asset in asset_names.iter() {
            let result: AssetInfo = QuerierWrapper::<CQ>::new(querier)
                .query::<AssetInfo>(&QueryRequest::Wasm(WasmQuery::Raw {
                    contract_addr: self.addr().into(),
                    key: Binary::from(concat(
                        // Query assets map
                        &to_length_prefixed(b"assets"),
                        asset.as_bytes(),
                    )),
                }))?;
    
            assets.insert(asset.to_string(), result);
        }
        Ok(assets.into_iter().map(|(v, k)| (v, k)).collect())
    }

    /// Query single contract address from Registry
    pub fn query_one_contract<Q, T, CQ>(&self, querier: &Q, contract_name: String) -> StdResult<Addr> 
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let result = QuerierWrapper::<CQ>::new(querier)
            .query::<String>(&QueryRequest::Wasm(WasmQuery::Raw {
                contract_addr: self.addr().to_string(),
                // query assets map
                key: Binary::from(concat(
                    &to_length_prefixed(b"contracts"),
                    contract_name.as_bytes(),
                )),
            }))?;
        // Addresses are checked when stored.
        Ok(Addr::unchecked(result))
    }


    /// Query single asset's info from the Registry contract
    pub fn query_one_asset<Q, T, CQ>(&self, querier: &Q, asset_name: String) -> StdResult<AssetInfo> 
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
    let result = QuerierWrapper::<CQ>::new(querier)
        .query::<AssetInfo>(&QueryRequest::Wasm(WasmQuery::Raw {
            contract_addr: self.addr().to_string(),
            // query assets map
            key: Binary::from(concat(
                &to_length_prefixed(b"assets"),
                asset_name.as_bytes(),
            )),
        }))?;
    Ok(result)
}
    
}
#[inline]
    fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
        let mut k = namespace.to_vec();
        k.extend_from_slice(key);
        k
    }
