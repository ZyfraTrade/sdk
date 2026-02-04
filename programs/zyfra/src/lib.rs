use anchor_lang::prelude::*;

declare_id!("ZyfrA1111111111111111111111111111111111");

#[program]
pub mod zyfra {
    use super::*;

    pub fn submit_insight(
        ctx: Context<SubmitInsight>,
        insight_hash: [u8; 32],
    ) -> Result<()> {
        let insight = &mut ctx.accounts.insight;

        insight.hash = insight_hash;
        insight.submitter = ctx.accounts.submitter.key();
        insight.score = 0;
        insight.timestamp = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn score_insight(
        ctx: Context<ScoreInsight>,
        score: u16,
    ) -> Result<()> {
        let insight = &mut ctx.accounts.insight;
        insight.score = score;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitInsight<'info> {
    #[account(init, payer = submitter, space = 8 + 32 + 32 + 2 + 8)]
    pub insight: Account<'info, Insight>,
    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ScoreInsight<'info> {
    #[account(mut)]
    pub insight: Account<'info, Insight>,
}

#[account]
pub struct Insight {
    pub hash: [u8; 32],
    pub submitter: Pubkey,
    pub score: u16,
    pub timestamp: i64,
}
