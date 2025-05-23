use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction::transfer}};
use anchor_safe_math::SafeMath;
use crate::{instructions::distribute::Distribute, staking::harvest};


#[event]
pub struct DistributeEvent {
  amount: u64,
  distributor: Pubkey,
  ts: i64,
}

pub fn exec(ctx: Context<Distribute>, amount: u64, _test_ts: i64) -> Result<()> {
  #[cfg(not(feature = "localnet"))]
  let now = Clock::get().unwrap().unix_timestamp;
  #[cfg(feature = "localnet")]
  let now = _test_ts;

  harvest(&mut ctx.accounts.pool_info, now)?;

  let pool_info = &mut ctx.accounts.pool_info;
  // we need to carry over any pending amount from the previous round to the next one.
  // This will be reflected in the `reward_per_sec` of the next round.
  // For example if round 1 has total rewards of 1000 but we call distribute mid way though with a new
  // amount of rewards being 2000 then the rewardsPerSec is not `2000 / round_duration_secs` but 
  // `(2000 + 500) / round_duration_secs`
  if now < pool_info.round_end_ts {
    let pending_from_current_round = pool_info.reward_per_sec.safe_mul((pool_info.round_end_ts - now) as u64)?;
    pool_info.reward_per_sec = amount
    .safe_add(pending_from_current_round)?
    .safe_div(pool_info.round_duration_secs as u64)?;
  } else {
    pool_info.reward_per_sec = amount.safe_div(pool_info.round_duration_secs as u64)?;
  }

  pool_info.last_harvest_ts = now;
  pool_info.round_end_ts = now + pool_info.round_duration_secs;

  send_sol_to_pool(&ctx, amount)?;

  emit_cpi!(DistributeEvent {
    amount,
    distributor: ctx.accounts.distributor.key(),
    ts: now,
  });

  Ok(())
}

fn send_sol_to_pool(ctx: &Context<Distribute>, amount: u64) -> Result<()> {
  let distributor = &ctx.accounts.distributor;
  let pool_info = &ctx.accounts.pool_info;

  invoke(
    &transfer(&distributor.key(), &pool_info.key(), amount),
    &[
      distributor.to_account_info(),
      pool_info.to_account_info(),
    ],
  )?;

  Ok(())
}
