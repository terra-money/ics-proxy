use cosmwasm_std::StdError;
use serde_json_wasm::ser::Error;
use thiserror::Error;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
}

impl From<serde_json_wasm::ser::Error> for ContractError {
    fn from(value: Error) -> Self {
        ContractError::Std(StdError::generic_err(value.to_string()))
    }
}
