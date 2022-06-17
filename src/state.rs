use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::Map;
use cw_asset::AssetInfo;

/// The ADMIN is the Addr that is able to modify things in the registry
pub const ADMIN: Admin = Admin::new("admin");

/// ASSET_MAP holds a mapping of user-provided strings which are linked to an instance of AssetInfo
/// This instance can contain any valid denom, amount and maybe a CW20, Native token or some IBC variant of either.
pub const ASSET_MAP: Map<&str, AssetInfo> = Map::new("assets");

/// CONTRACT_MAP holds a mapping of user-provided strings to some validated address.
/// This address could be a contract in your suite of contracts allowing for easy contract 
/// replace in cases where an upgrade is not applicable or an EOA that is often interacted with.
pub const CONTRACT_MAP: Map<&str, Addr> = Map::new("contracts");
