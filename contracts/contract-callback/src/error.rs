use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Invalid Payment {amount}")]
    InvalidPayment {amount: Uint128},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unknown Job ID")]
    UnknownJobId {},

    #[error("Job ID not found")]
    NotFoundJobId {},
}
