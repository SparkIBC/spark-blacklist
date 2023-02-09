use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

/// SparkBlacklistContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SparkBlacklistContract(pub Addr);

impl SparkBlacklistContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }
}
