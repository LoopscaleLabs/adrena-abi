use {
    crate::{
        math,
        types::{Custody, Position},
        Cortex, Side,
    },
    anyhow::Result,
};

pub fn get_liquidation_price(
    position: &Position,
    custody: &Custody,
    collateral_custody: &Custody,
    current_time: i64,
) -> Result<u64> {
    // liq_price = pos_price +- (collateral + unreal_profit - unreal_loss - exit_fee - interest - size/max_leverage) * pos_price / size

    if position.size_usd == 0 || position.price == 0 {
        return Ok(0);
    }

    let total_unrealized_interest_usd = collateral_custody
        .get_interest_amount_usd(position, current_time)?
        + position.unrealized_interest_usd;
    let unrealized_loss_usd = position.liquidation_fee_usd + total_unrealized_interest_usd;

    let mut max_loss_usd = math::checked_as_u64(
        (position.size_usd as u128 * Cortex::BPS_POWER) / custody.pricing.max_leverage as u128,
    )?;

    max_loss_usd += unrealized_loss_usd;

    let margin_usd = position.collateral_usd;

    let max_price_diff = if max_loss_usd >= margin_usd {
        max_loss_usd - margin_usd
    } else {
        margin_usd - max_loss_usd
    };

    let max_price_diff = math::scale_to_exponent(
        max_price_diff,
        -(Cortex::USD_DECIMALS as i32),
        -(Cortex::PRICE_DECIMALS as i32),
    )?;

    let position_size_usd = math::scale_to_exponent(
        position.size_usd,
        -(Cortex::USD_DECIMALS as i32),
        -(Cortex::PRICE_DECIMALS as i32),
    )?;

    let max_price_diff = math::checked_as_u64(
        (max_price_diff as u128 * position.price as u128) / position_size_usd as u128,
    )?;

    if position.get_side() == Side::Long {
        if max_loss_usd >= margin_usd {
            Ok(position.price + max_price_diff)
        } else if position.price > max_price_diff {
            Ok(position.price - max_price_diff)
        } else {
            Ok(0)
        }
    } else if max_loss_usd >= margin_usd {
        if position.price > max_price_diff {
            Ok(position.price - max_price_diff)
        } else {
            Ok(0)
        }
    } else {
        Ok(position.price + max_price_diff)
    }
}

// Returns the interest amount that has accrued since the last position cumulative interest snapshot update
pub fn get_interest_amount_usd(
    custody: &Custody,
    position: &Position,
    current_time: i64,
) -> Result<u64> {
    if position.borrow_size_usd == 0 {
        return Ok(0);
    }

    let cumulative_interest = get_cumulative_interest(custody, current_time)?;

    let position_interest = if cumulative_interest > position.cumulative_interest_snapshot.to_u128()
    {
        cumulative_interest - position.cumulative_interest_snapshot.to_u128()
    } else {
        return Ok(0);
    };

    Ok(math::checked_as_u64(
        (position_interest * position.borrow_size_usd as u128) / Cortex::RATE_POWER,
    )?)
}

pub fn get_cumulative_interest(custody: &Custody, current_time: i64) -> Result<u128> {
    if current_time > custody.borrow_rate_state.last_update {
        let cumulative_interest = math::checked_ceil_div(
            (current_time - custody.borrow_rate_state.last_update) as u128
                * custody.borrow_rate_state.current_rate as u128,
            3_600,
        )?;

        Ok(custody.borrow_rate_state.cumulative_interest.to_u128() + cumulative_interest)
    } else {
        Ok(custody.borrow_rate_state.cumulative_interest.to_u128())
    }
}
