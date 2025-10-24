use bolt_lang::*;
use anchor_lang::prelude::*;
use trading_account::TradingAccount;
use position::{Position, PositionType};
use leaderboard::{Leaderboard, LeaderboardEntry};

declare_id!("49kLMtwwnm5wdCKvtToUYgoxxo9PXjheS1rwcoSYkQfG");

#[system]
pub mod close_position {

    pub fn execute(ctx: Context<Components>, args: Args) -> Result<Components> {
        let trading_account = &mut ctx.accounts.trading_account;
        let position = &mut ctx.accounts.position;
        let leaderboard = &mut ctx.accounts.leaderboard;
        let player = ctx.accounts.authority.key();
        let clock = Clock::get()?;

        require!(
            position.owner == player,
            ErrorCode::UnauthorizedPlayer
        );

        require!(
            position.is_active,
            ErrorCode::PositionAlreadyClosed
        );

        let entry_price = position.entry_price as i64;
        let exit_price = args.exit_price as i64;
        let size = position.size as i64;

        let pnl = match position.direction {
            PositionType::Long => {
                ((exit_price - entry_price) * size) / entry_price
            },
            PositionType::Short => {
                ((entry_price - exit_price) * size) / entry_price
            },
        };

        let return_amount = if pnl >= 0 {
            (size as u64).checked_add(pnl as u64).unwrap()
        } else {
            let loss = pnl.abs() as u64;
            if loss >= size as u64 {
                0
            } else {
                (size as u64).checked_sub(loss).unwrap()
            }
        };

        trading_account.virtual_balance = trading_account.virtual_balance
            .checked_add(return_amount).unwrap();
        trading_account.realized_pnl = trading_account.realized_pnl
            .checked_add(pnl).unwrap();
        trading_account.active_positions = trading_account.active_positions
            .checked_sub(1).unwrap();
        trading_account.total_trades = trading_account.total_trades
            .checked_add(1).unwrap();

        if pnl > 0 {
            trading_account.winning_trades = trading_account.winning_trades.checked_add(1).unwrap();
        } else if pnl < 0 {
            trading_account.losing_trades = trading_account.losing_trades.checked_add(1).unwrap();
        }

        position.is_active = false;

        if let Some(entry) = leaderboard.entries.iter_mut()
            .find(|e| e.player == player) {
            entry.pnl = trading_account.realized_pnl;
            leaderboard.last_updated = clock.unix_timestamp;
        }

        leaderboard.entries.sort_by(|a, b| b.pnl.cmp(&a.pnl));
        for (idx, entry) in leaderboard.entries.iter_mut().enumerate() {
            entry.rank = (idx + 1) as u32;
        }

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub trading_account: TradingAccount,
        pub position: Position,
        pub leaderboard: Leaderboard,
    }

    #[arguments]
    pub struct Args {
        pub exit_price: u64,
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized player")]
    UnauthorizedPlayer,
    #[msg("Position already closed")]
    PositionAlreadyClosed,
}
