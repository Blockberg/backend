use bolt_lang::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use competition::{Competition, CompetitionState};
use trading_account::TradingAccount;
use leaderboard::{Leaderboard, LeaderboardEntry};

declare_id!("FFRL7nSQxFYMEcUxb912WsvbMSDMPNefdgCe4aZYNxWk");

#[system]
pub mod join_competition {

    pub fn execute(ctx: Context<Components>, _args: Vec<u8>) -> Result<Components> {
        let competition = &mut ctx.accounts.competition;
        let trading_account = &mut ctx.accounts.trading_account;
        let leaderboard = &mut ctx.accounts.leaderboard;
        let player = ctx.accounts.authority.key();
        let clock = Clock::get()?;

        require!(
            competition.state == CompetitionState::Active ||
            (competition.state == CompetitionState::Pending && clock.unix_timestamp >= competition.start_time),
            ErrorCode::CompetitionNotActive
        );

        require!(
            clock.unix_timestamp < competition.end_time,
            ErrorCode::CompetitionEnded
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            competition.entry_fee,
        )?;

        trading_account.owner = player;
        trading_account.competition = competition.key();
        trading_account.virtual_balance = competition.entry_fee;
        trading_account.realized_pnl = 0;
        trading_account.unrealized_pnl = 0;
        trading_account.total_trades = 0;
        trading_account.winning_trades = 0;
        trading_account.losing_trades = 0;
        trading_account.active_positions = 0;

        competition.total_participants = competition.total_participants.checked_add(1).unwrap();
        competition.prize_pool = competition.prize_pool.checked_add(competition.entry_fee).unwrap();

        if competition.state == CompetitionState::Pending {
            competition.state = CompetitionState::Active;
        }

        leaderboard.entries.push(LeaderboardEntry {
            player,
            pnl: 0,
            rank: competition.total_participants,
        });
        leaderboard.last_updated = clock.unix_timestamp;

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
        pub trading_account: TradingAccount,
        pub leaderboard: Leaderboard,
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Competition is not active yet")]
    CompetitionNotActive,
    #[msg("Competition has ended")]
    CompetitionEnded,
}
