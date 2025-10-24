use bolt_lang::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{mint_to, MintTo, Mint, Token, TokenAccount};
use competition::{Competition, CompetitionState};
use leaderboard::Leaderboard;

declare_id!("C1FTdtq531t4MViYtgo7LAft3GRkJimYAhVWFU4BE46i");

#[system]
pub mod settle_competition {

    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let competition = &mut ctx.accounts.competition;
        let leaderboard = &ctx.accounts.leaderboard;
        let clock = Clock::get()?;

        require!(
            competition.state == CompetitionState::Active ||
            competition.state == CompetitionState::Ended,
            ErrorCode::CompetitionNotActive
        );

        require!(
            clock.unix_timestamp >= competition.end_time,
            ErrorCode::CompetitionNotEnded
        );

        require!(
            ctx.accounts.authority.key() == competition.admin,
            ErrorCode::UnauthorizedAdmin
        );

        if leaderboard.entries.is_empty() {
            competition.state = CompetitionState::Settled;
            return Ok(ctx.accounts);
        }

        let winner_entry = &leaderboard.entries[0];
        competition.winner = Some(winner_entry.player);
        competition.state = CompetitionState::Settled;

        let prize_amount = competition.prize_pool
            .checked_mul(80).unwrap()
            .checked_div(100).unwrap();

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.winner.to_account_info(),
                },
            ),
            prize_amount,
        )?;

        if let Some(nft_mint) = &ctx.accounts.nft_mint {
            mint_to(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: nft_mint.to_account_info(),
                        to: ctx.accounts.winner_token_account.as_ref().unwrap().to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                    },
                ),
                1,
            )?;
        }

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
        pub leaderboard: Leaderboard,
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Competition is not in correct state")]
    CompetitionNotActive,
    #[msg("Competition has not ended yet")]
    CompetitionNotEnded,
    #[msg("Only admin can settle competition")]
    UnauthorizedAdmin,
}
