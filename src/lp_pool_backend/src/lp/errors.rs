use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum Errors {
    InvalidPrice,
    InvalidFee,
    InvalidLiquidityTarget,
    InvalidDeposit,
    InvalidSwapAmount,
    InvalidLpTokenToRemove,
    ToBigSwap(u64),
}
