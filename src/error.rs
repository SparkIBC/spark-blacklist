use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error")]
    CustomError {},

    #[error("Custom Error val: {val:?}")]
    CustomErrorParam { val: String },
}
