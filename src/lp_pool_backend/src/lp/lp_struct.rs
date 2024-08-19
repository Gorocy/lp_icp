use crate::lp::calc::{divide, multiply};
use crate::lp::errors::Errors;
use crate::lp::var_type::{LpTokenAmount, Percentage, Price, StakedTokenAmount, TokenAmount};

pub struct LpPool {
    price: Price,
    token_amount_reserve: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

impl LpPool {
    pub fn init(
        price: Price,
        min_fee: Percentage,
        max_fee: Percentage,
        liquidity_target: TokenAmount,
    ) -> Result<Self, Errors> {
        // min_fee have to be smaller or equal, but I assume in this model only smaller
        if min_fee.0 >= max_fee.0 {
            return Err(Errors::InvalidFee);
        }
        // price grater that 0
        if price.0 == 0 {
            return Err(Errors::InvalidPrice);
        }
        // target should be greater than 0
        // if target == 0, fee would be a constant
        if liquidity_target.0 == 0 {
            return Err(Errors::InvalidLiquidityTarget);
        }

        Ok(LpPool {
            price,
            token_amount_reserve: TokenAmount(0),
            st_token_amount: StakedTokenAmount(0),
            lp_token_amount: LpTokenAmount(0),
            liquidity_target,
            min_fee,
            max_fee,
        })
    }

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Errors> {
        // deposit must be grater than 0
        if token_amount.0 == 0 {
            return Err(Errors::InvalidDeposit);
        }

        // if token == lp_token and st_token == 0, then
        // calculating is unnecessary
        if self.lp_token_amount.0 == self.token_amount_reserve.0 && self.st_token_amount.0 == 0 {
            self.token_amount_reserve.0 += token_amount.0;
            self.lp_token_amount.0 += token_amount.0;
            return Ok(LpTokenAmount(token_amount.0));
        }

        // value of tokens and st_tokens in tokens before deposit 
        let value_token =
            self.token_amount_reserve.0 + multiply(self.st_token_amount.0, self.price.0);
        // value of tokens and st_tokens in tokens after deposit 
        let new_value_token = value_token + token_amount.0;
        // proportion after to before
        let token_proportion = divide(new_value_token, value_token);
        // we need to receive the same proportion of lp_tokens before to lp_tokens after
        let lp_token_minted =
            multiply(self.lp_token_amount.0, token_proportion) - self.lp_token_amount.0;
        
        // deposit tokens
        self.token_amount_reserve.0 += token_amount.0;
        // saving information about lp_tokens existing
        self.lp_token_amount.0 += lp_token_minted;

        Ok(LpTokenAmount(lp_token_minted))
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        // cannot withdraw for 0 or more than exist(it shouldn't happen ever, but it is simulator)
        if lp_token_amount.0 == 0 || lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InvalidLpTokenToRemove)
        }
        // what part send of existing
        let part_of_lp = divide(lp_token_amount.0, self.lp_token_amount.0);

        // update existing amount
        self.lp_token_amount.0 -= lp_token_amount.0;

        // calculating proportion for token and update amount in pool
        let token_to_receive = multiply(part_of_lp, self.token_amount_reserve.0);
        self.token_amount_reserve.0 -= token_to_receive;

        // calculating proportion for st_token and update amount in pool
        let st_token_to_receive = multiply(part_of_lp, self.st_token_amount.0);
        self.st_token_amount.0 -= st_token_to_receive;

        Ok((
            TokenAmount(token_to_receive),
            StakedTokenAmount(st_token_to_receive),
        ))
    }

    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Errors> {
        // cannot swap 0
        if staked_token_amount.0 == 0 {
            return Err(Errors::InvalidSwapAmount);
        }

        // token for send st_token amount
        let token_for_st = multiply(self.price.0, staked_token_amount.0);

        // check amount after max_fee
        let token_receive_max_fee = token_for_st - multiply(token_for_st, self.max_fee.0);

        // cannot swap if pool don't have enough
        if token_receive_max_fee > self.token_amount_reserve.0 {
            // check biggest possible amount for token with max_fee
            let token_max_fee =
                self.token_amount_reserve.0 - multiply(self.token_amount_reserve.0, self.max_fee.0);
            let st_max_swap = divide(token_max_fee, self.price.0);
            return Err(Errors::ToBigSwap(st_max_swap));
        }

        // check amount after min_fee
        let token_min_fee = token_for_st - multiply(token_for_st, self.min_fee.0);
        let check_target = self.token_amount_reserve.0 - token_min_fee;

        // there will be more tokens left in the pool than target
        if check_target > self.liquidity_target.0 || self.min_fee.0 == self.max_fee.0 {
            self.st_token_amount.0 += staked_token_amount.0;
            self.token_amount_reserve.0 -= token_min_fee;
            return Ok(TokenAmount(token_min_fee));
        }

        // amount will be,
        // maybe it should be checked with token_for_st - min_fee,
        // but it would pass history
        let amount_after = self.token_amount_reserve.0 - token_for_st;

        // proportion amount after to target
        let target_part = divide(amount_after, self.liquidity_target.0);

        // calculate the unstake fee using the formula:
        // unstake_fee = maxFee - (maxFee - minFee) * amountAfter / target

        // this formula computes the fee based on the difference between the maximum and minimum fee,
        // proportionally adjusted by the ratio of amount after to target.
        let unstake_fee = self.max_fee.0 - multiply(self.max_fee.0 - self.min_fee.0, target_part);

        // tokens user will receive after fee
        let token_to_receive = token_for_st - multiply(token_for_st, unstake_fee);

        // update amount of tokens
        self.token_amount_reserve.0 -= token_to_receive;
        // update amount of st_tokens
        self.st_token_amount.0 += staked_token_amount.0;

        Ok(TokenAmount(token_to_receive))
    }


    pub fn get_price(&self) -> Price {
        self.price
    }

    pub fn get_token_amount(&self) -> TokenAmount {
        self.token_amount_reserve
    }

    pub fn get_st_token_amount(&self) -> StakedTokenAmount {
        self.st_token_amount
    }

    pub fn get_lp_token_amount(&self) -> LpTokenAmount {
        self.lp_token_amount
    }

    pub fn get_target(&self) -> TokenAmount {
        self.liquidity_target
    }

}
