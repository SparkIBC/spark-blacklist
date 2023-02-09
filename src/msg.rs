use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Status;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum SetStatusMsg {
    AllowAll,
    AllowNone,
    AllowSome(Vec<Addr>),
}

#[cw_serde]
pub enum ExecuteMsg {
    SetStatus { status: SetStatusMsg },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StatusResponse)]
    GetStatus { address: Addr },

    #[returns(AllowanceResponse)]
    GetAllowance {
        /// Address allowing or disallowing donations
        address: Addr,
        /// Address attempting to donate on behalf of `address`
        sender: Addr,
    },
}

// We define a custom struct for each query response

#[cw_serde]
pub struct StatusResponse {
    pub status: Option<Status>,
}

#[cw_serde]
pub struct AllowanceResponse {
    pub allowance: bool,
}
