// programs/zyfra/src/lib.rs
use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("Zyfra1111111111111111111111111111111111");

#[program]
pub mod zyfra {
    use super::*;

    pub fn submit(ctx: Context<Submit>, hash: [u8; 32]) -> Result<()> {
        handler(ctx, hash)
    }
}
