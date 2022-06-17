use cosmwasm_std::StdError;
use cw_controllers::AdminError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("Found no associated address or asset with stored name {val:?}")]
    NotFoundInRegistry { val: String },
}
