use cosmwasm_std::StdError;

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
    fn from(value: serde_json_wasm::ser::Error) -> Self {
        ContractError::Std(StdError::generic_err(value.to_string()))
    }
}

impl From<bech32_no_std::Error> for ContractError {
    fn from(value: bech32_no_std::Error) -> Self {
        ContractError::Std(StdError::generic_err(value.to_string()))
    }
}
