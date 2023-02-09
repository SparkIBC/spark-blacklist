use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[cw_serde]
pub enum Status {
    AllowNone,
    AllowSome(Vec<Addr>),
}

pub const STATUS_LIST: Map<&Addr, Status> = Map::new("status_list");
