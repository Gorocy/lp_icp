use candid::Principal;

use std::cell::RefCell;

mod lp;
use crate::lp::lp_struct::*;
use crate::lp::errors::Errors;

use crate::lp::var_type::{LpTokenAmount, Percentage, Price, StakedTokenAmount, TokenAmount};

thread_local! {
    static THREAD_LOCAL_STRUCT: RefCell<LpPool> = RefCell::new(LpPool::init(
        Price(1__500_000),
        Percentage(1_000),
        Percentage(90_000),
        TokenAmount(90__000_000),).unwrap());

}

// type User = HashMap<Principal, u64>;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn lp_price() -> u64 {
    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        lp_pool.borrow().get_price().0
    })
}

#[ic_cdk::query]
fn lp_token_amount() -> u64 {
    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        lp_pool.borrow().get_token_amount().0
    })
}

#[ic_cdk::query]
fn lp_st_token_amount() -> u64 {
    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        lp_pool.borrow().get_st_token_amount().0        
    })
}

#[ic_cdk::query]
fn lp_lp_token_amount() -> u64 {
    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        lp_pool.borrow().get_lp_token_amount().0
    })
}

#[ic_cdk::query]
fn lp_target_amount() -> u64 {
    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        lp_pool.borrow().get_target().0
    })
}

#[ic_cdk::update]
fn add_liquidity_to_pool(token_amount: u64) -> Result<u64, Errors> {
    let token_amount = TokenAmount(token_amount);

    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        let mut lp_pool = lp_pool.borrow_mut();
        lp_pool.add_liquidity(token_amount).map(|lp_token_amount| lp_token_amount.0)
    })
}

#[ic_cdk::update]
fn remove_liquidity_from_pool(lp_amount: u64) -> Result<(u64, u64), Errors> {
    let lp_amount = LpTokenAmount(lp_amount);

    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        let mut lp_pool = lp_pool.borrow_mut();
        lp_pool
            .remove_liquidity(lp_amount)
            .map(|(token_amount, st_token_amount) | (token_amount.0, st_token_amount.0))
    })
}

#[ic_cdk::update]
fn swap_st_pool(st_token_amount: u64) -> Result<u64, Errors> {
    let st_token_amount = StakedTokenAmount(st_token_amount);

    THREAD_LOCAL_STRUCT.with(|lp_pool| {
        let mut lp_pool = lp_pool.borrow_mut();
        lp_pool.swap(st_token_amount).map(|token_amount| token_amount.0)
    })
}

ic_cdk::export_candid!();
