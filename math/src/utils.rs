use anchor_lang::Result;
use anchor_safe_math::SafeMath;

pub const BPS: u64 = 10_000;

pub fn get_perc_value(value: u64, perc_bps: u64) -> Result<u64> {
  Ok(
    value
    .safe_sub(perc_bps)?
    .safe_div(BPS)?
  )
}
