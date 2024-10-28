use {
    super::BPS_POWER,
    crate::U128Split,
    anyhow::{anyhow, Result},
    std::{
        fmt::Display,
        ops::{AddAssign, SubAssign},
    },
};

pub fn checked_float_div<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::Float + Display,
{
    if arg2 == T::zero() {
        log::info!("Error: Overflow in {} / {}", arg1, arg2);
        return Err(anyhow!("Math overflow"));
    }

    let res = arg1 / arg2;

    if !res.is_finite() {
        log::info!("Error: Overflow in {} / {}", arg1, arg2);
        Err(anyhow!("Math overflow"))
    } else {
        Ok(res)
    }
}

pub fn checked_ceil_div<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::PrimInt + Display + num_traits::Unsigned,
{
    if arg1 > T::zero() {
        if arg1 == arg2 && arg2 != T::zero() {
            return Ok(T::one());
        }

        if let Some(res) = (arg1 - T::one()).checked_div(&arg2) {
            Ok(res + T::one())
        } else {
            log::info!("Error: Overflow in {} / {}", arg1, arg2);
            Err(anyhow!("Math overflow"))
        }
    } else if let Some(res) = arg1.checked_div(&arg2) {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} / {}", arg1, arg2);
        Err(anyhow!("Math overflow"))
    }
}
pub fn checked_decimal_div(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    if coefficient2 == 0 {
        log::info!("Error: Overflow in {} / {}", coefficient1, coefficient2);
        return Err(anyhow!("Math overflow"));
    }

    if coefficient1 == 0 {
        return Ok(0);
    }

    // compute scale factor for the dividend
    let mut scale_factor = 0;
    let mut target_power = exponent1 - exponent2 - target_exponent;

    if exponent1 > 0 {
        scale_factor += exponent1;
        target_power -= exponent1;
    }

    if exponent2 < 0 {
        scale_factor -= exponent2;
        target_power += exponent2;
    }

    if target_exponent < 0 {
        scale_factor -= target_exponent;
        target_power += target_exponent;
    }

    let scaled_coeff1 = if scale_factor > 0 {
        coefficient1 as u128 * checked_pow(10u128, scale_factor as usize)?
    } else {
        coefficient1 as u128
    };

    if target_power >= 0 {
        checked_as_u64(
            (scaled_coeff1 / coefficient2 as u128) * checked_pow(10u128, target_power as usize)?,
        )
    } else {
        checked_as_u64(
            (scaled_coeff1 / coefficient2 as u128) / checked_pow(10u128, (-target_power) as usize)?,
        )
    }
}

pub fn checked_decimal_ceil_div(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    if coefficient2 == 0 {
        log::info!("Error: Overflow in {} / {}", coefficient1, coefficient2);
        return Err(anyhow!("Math overflow"));
    }

    if coefficient1 == 0 {
        return Ok(0);
    }

    // Compute scale factor for the dividend
    let mut scale_factor = 0;
    let mut target_power = exponent1 - exponent2 - target_exponent;

    if exponent1 > 0 {
        scale_factor += exponent1;
        target_power += exponent1;
    }

    if exponent2 < 0 {
        scale_factor -= exponent2;
        target_power += exponent2;
    }

    if target_exponent < 0 {
        scale_factor -= target_exponent;
        target_power += target_exponent;
    }

    let scaled_coeff1 = if scale_factor > 0 {
        coefficient1 as u128 * checked_pow(10u128, scale_factor as usize)?
    } else {
        coefficient1 as u128
    };

    if target_power >= 0 {
        checked_as_u64(
            checked_ceil_div(scaled_coeff1, coefficient2 as u128)?
                * checked_pow(10u128, target_power as usize)?,
        )
    } else {
        checked_as_u64(
            checked_ceil_div(scaled_coeff1, coefficient2 as u128)?
                / checked_pow(10u128, (-target_power) as usize)?,
        )
    }
}

pub fn checked_token_div(
    amount1: u64,
    decimals1: u8,
    amount2: u64,
    decimals2: u8,
) -> Result<(u64, u8)> {
    let target_decimals = std::cmp::max(decimals1, decimals2);
    Ok((
        checked_decimal_div(
            amount1,
            -(decimals1 as i32),
            amount2,
            -(decimals2 as i32),
            -(target_decimals as i32),
        )?,
        target_decimals,
    ))
}

pub fn checked_float_mul<T>(arg1: T, arg2: T) -> Result<T>
where
    T: num_traits::Float + Display,
{
    let res = arg1 * arg2;

    if !res.is_finite() {
        log::info!("Error: Overflow in {} * {}", arg1, arg2);
        Err(anyhow!("Math overflow"))
    } else {
        Ok(res)
    }
}

pub fn checked_decimal_mul(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    if coefficient1 == 0 || coefficient2 == 0 {
        return Ok(0);
    }

    let target_power = (exponent1 + exponent2) - target_exponent;

    if target_power >= 0 {
        checked_as_u64(
            (coefficient1 as u128 * coefficient2 as u128)
                * checked_pow(10u128, target_power as usize)?,
        )
    } else {
        checked_as_u64(
            (coefficient1 as u128 * coefficient2 as u128)
                / checked_pow(10u128, (-target_power) as usize)?,
        )
    }
}

pub fn checked_decimal_ceil_mul(
    coefficient1: u64,
    exponent1: i32,
    coefficient2: u64,
    exponent2: i32,
    target_exponent: i32,
) -> Result<u64> {
    if coefficient1 == 0 || coefficient2 == 0 {
        return Ok(0);
    }

    let target_power = (exponent1 + exponent2) - target_exponent;

    if target_power >= 0 {
        checked_as_u64(
            (coefficient1 as u128 * coefficient2 as u128)
                * checked_pow(10u128, target_power as usize)?,
        )
    } else {
        checked_as_u64(checked_ceil_div(
            coefficient1 as u128 * coefficient2 as u128,
            checked_pow(10u128, (-target_power) as usize)?,
        )?)
    }
}

pub fn checked_token_mul(
    amount1: u64,
    decimals1: u8,
    amount2: u64,
    decimals2: u8,
) -> Result<(u64, u8)> {
    let target_decimals = std::cmp::max(decimals1, decimals2);

    Ok((
        checked_decimal_mul(
            amount1,
            -(decimals1 as i32),
            amount2,
            -(decimals2 as i32),
            -(target_decimals as i32),
        )?,
        target_decimals,
    ))
}

pub fn checked_pow<T>(arg: T, exp: usize) -> Result<T>
where
    T: num_traits::PrimInt + Display,
{
    if let Some(res) = num_traits::checked_pow(arg, exp) {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} ^ {}", arg, exp);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_powf(arg: f64, exp: f64) -> Result<f64> {
    let res = f64::powf(arg, exp);

    if res.is_finite() {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} ^ {}", arg, exp);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_powi(arg: f64, exp: i32) -> Result<f64> {
    let res = if exp > 0 {
        f64::powi(arg, exp)
    } else {
        // Workaround due to f64::powi() not working properly on-chain with negative exponent
        checked_float_div(1.0, f64::powi(arg, -exp))?
    };

    if res.is_finite() {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} ^ {}", arg, exp);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_usize<T>(arg: T) -> Result<usize>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<usize> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as usize", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_u16<T>(arg: T) -> Result<u16>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<u16> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as u16", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_i32<T>(arg: T) -> Result<i32>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<i32> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as i32", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_i64<T>(arg: T) -> Result<i64>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<i64> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as i64", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_u64<T>(arg: T) -> Result<u64>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<u64> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as u64", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_u128<T>(arg: T) -> Result<u128>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<u128> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as u128", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn checked_as_f64<T>(arg: T) -> Result<f64>
where
    T: Display + num_traits::ToPrimitive + Clone,
{
    let option: Option<f64> = num_traits::NumCast::from(arg.clone());

    if let Some(res) = option {
        Ok(res)
    } else {
        log::info!("Error: Overflow in {} as f64", arg);
        Err(anyhow!("Math overflow"))
    }
}

pub fn scale_to_exponent(arg: u64, exponent: i32, target_exponent: i32) -> Result<u64> {
    if target_exponent == exponent {
        return Ok(arg);
    }

    let delta = target_exponent - exponent;

    if delta > 0 {
        // Arg's number of decimals is shrinking
        let result = arg / checked_pow(10, delta as usize)?;

        // Accept 1bps loss of precision
        if result * checked_pow(10, delta as usize)? < (arg - (arg / BPS_POWER as u64)) {
            return Err(anyhow!(
                "Pyth price exponent too large incurring precision loss"
            ));
        }

        return Ok(result);
    }

    // arg's number of decimals is increasing, there is no loss of precision
    Ok(arg * checked_pow(10, (-delta) as usize)?)
}

pub fn to_ui_amount(amount: u64, decimals: u8) -> Result<f64> {
    checked_float_div(
        checked_as_f64(amount)?,
        checked_powi(10.0, decimals as i32)?,
    )
}

pub fn to_token_amount(ui_amount: f64, decimals: u8) -> Result<u64> {
    checked_as_u64(checked_float_mul(
        ui_amount,
        checked_powi(10.0, decimals as i32)?,
    )?)
}

// U128 SPLIT
impl U128Split {
    // Create a new U128Split from a u128
    pub fn new(value: u128) -> Self {
        let high = (value >> 64) as u64;
        let low = value as u64;
        U128Split { high, low }
    }

    // Convert the U128Split back into a u128
    pub fn to_u128(&self) -> u128 {
        ((self.high as u128) << 64) | (self.low as u128)
    }

    // Add a u128 to this U128Split
    pub fn add(&mut self, other: u128) {
        let other_split = U128Split::new(other);
        let (low, carry) = self.low.overflowing_add(other_split.low);
        let high = self.high + other_split.high + (carry as u64);
        self.high = high;
        self.low = low;
    }

    // Method to perform left shift operation
    pub fn left_shift(&mut self, shift: u32) {
        if shift == 0 {
            return;
        }

        let total_bits = 128;
        let u64_bits = 64;

        if shift >= total_bits {
            self.high = 0;
            self.low = 0;
        } else if shift >= u64_bits {
            self.high = self.low << (shift - u64_bits);
            self.low = 0;
        } else {
            self.high = (self.high << shift) | (self.low >> (u64_bits - shift));
            self.low <<= shift;
        }
    }
}

impl std::ops::Add for U128Split {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self.high + other.high + (carry as u64);
        U128Split { high, low }
    }
}

impl std::ops::Sub for U128Split {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (low, borrow) = self.low.overflowing_sub(other.low);
        let high = self.high - other.high - (borrow as u64);
        U128Split { high, low }
    }
}

impl From<u128> for U128Split {
    fn from(item: u128) -> Self {
        U128Split::new(item)
    }
}

impl From<u64> for U128Split {
    fn from(item: u64) -> Self {
        U128Split::new(item as u128)
    }
}

impl From<i32> for U128Split {
    fn from(item: i32) -> Self {
        U128Split::new(item as u128)
    }
}

impl AddAssign for U128Split {
    fn add_assign(&mut self, other: Self) {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self.high + other.high + (carry as u64);
        self.high = high;
        self.low = low;
    }
}

impl AddAssign<u64> for U128Split {
    fn add_assign(&mut self, other: u64) {
        let (low, carry) = self.low.overflowing_add(other);
        self.high += carry as u64; // Only add to high if there's a carry
        self.low = low;
    }
}

impl AddAssign<u128> for U128Split {
    fn add_assign(&mut self, other: u128) {
        let other_split = U128Split::new(other);
        self.add_assign(other_split);
    }
}

impl SubAssign for U128Split {
    fn sub_assign(&mut self, other: Self) {
        let (low, borrow) = self.low.overflowing_sub(other.low);
        let high = self.high - other.high - (borrow as u64);
        self.high = high;
        self.low = low;
    }
}

impl SubAssign<u64> for U128Split {
    fn sub_assign(&mut self, other: u64) {
        let (low, borrow) = self.low.overflowing_sub(other);
        self.high -= borrow as u64; // Only subtract from high if there's a borrow
        self.low = low;
    }
}

impl SubAssign<u128> for U128Split {
    fn sub_assign(&mut self, other: u128) {
        let other_split = U128Split::new(other);
        self.sub_assign(other_split);
    }
}
