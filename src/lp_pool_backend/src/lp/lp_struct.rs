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
        if min_fee.0 > max_fee.0 {
            return Err(Errors::InvalidFee);
        }

        if price.0 == 0 {
            return Err(Errors::InvalidPrice);
        }

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
        if token_amount.0 == 0 {
            return Err(Errors::InvalidDeposit);
        }

        if self.lp_token_amount.0 == self.token_amount_reserve.0 && self.st_token_amount.0 == 0  {
            self.token_amount_reserve.0 += token_amount.0;
            self.lp_token_amount.0 += token_amount.0;
            return Ok(LpTokenAmount(token_amount.0));
        }

        let value_token =
            self.token_amount_reserve.0 + multiply(self.st_token_amount.0, self.price.0);
        let new_value_token = value_token + token_amount.0;
        let token_propotion = divide(new_value_token, value_token);
        let lp_token_minted =
            multiply(self.lp_token_amount.0, token_propotion) - self.lp_token_amount.0;

        self.token_amount_reserve.0 += token_amount.0;
        self.lp_token_amount.0 += lp_token_minted;

        Ok(LpTokenAmount(lp_token_minted))
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        if lp_token_amount.0 == 0 || lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InvalidLpTokenToRemove);
        }

        let part_of_lp = divide(lp_token_amount.0, self.lp_token_amount.0);

        self.lp_token_amount.0 -= lp_token_amount.0;

        let token_to_receive = multiply(part_of_lp, self.token_amount_reserve.0);
        self.token_amount_reserve.0 -= token_to_receive;

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
        if staked_token_amount.0 == 0 {
            return Err(Errors::InvalidSwapAmount);
        }

        let token_for_st = multiply(self.price.0, staked_token_amount.0);

        // IF REVERVE < 0
        let token_receive_max_fee = token_for_st - multiply(token_for_st, self.max_fee.0);

        if token_receive_max_fee > self.token_amount_reserve.0 {
            let token_max_fee =
                self.token_amount_reserve.0 - multiply(self.token_amount_reserve.0, self.max_fee.0);
            let st_max_swap = divide(token_max_fee, self.price.0);
            return Err(Errors::ToBigSwap(st_max_swap));
        }

        let token_min_fee = token_for_st - multiply(token_for_st, self.min_fee.0);
        let check_target = self.token_amount_reserve.0 - token_min_fee;

        if check_target > self.liquidity_target.0 || self.min_fee.0 == self.max_fee.0 {
            self.st_token_amount.0 += staked_token_amount.0;
            self.token_amount_reserve.0 -= token_min_fee;
            return Ok(TokenAmount(token_min_fee));
        }

        let amount_after = self.token_amount_reserve.0 - token_for_st; // simple diffrence without fee

        let target_part = divide(amount_after, self.liquidity_target.0);
        let unstake_fee = self.max_fee.0 - multiply(self.max_fee.0 - self.min_fee.0, target_part);
        let token_to_receive = token_for_st - multiply(token_for_st, unstake_fee);
        self.token_amount_reserve.0 -= token_to_receive;
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