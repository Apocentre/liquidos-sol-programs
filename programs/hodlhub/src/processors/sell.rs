use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::{invoke, invoke_signed}, system_instruction::transfer
  },
};

pub fn exec(
  mut ctx: Context<Sell>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  
}
