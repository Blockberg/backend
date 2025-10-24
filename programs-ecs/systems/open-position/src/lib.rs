use bolt_lang::*;
use anchor_lang::prelude::*;
use competition::{Competition, CompetitionState};
use trading_account::TradingAccount;
use position::{Position, PositionType};

declare_id!("B1LMnYAtxvQLFG56YS9vscBFLid7KHp1nWTPYtFKFLPh");

#[system]
pub mod open_position {

    pub fn execute(ctx: Context<Components>, args: Args) -> Result<Components> {
        let competition = &ctx.accounts.competition;
        let trading_account = &mut ctx.accounts.trading_account;
        let position = &mut ctx.accounts.position;
        let player = ctx.accounts.authority.key();
        let clock = Clock::get()?;

        require!(
            competition.state == CompetitionState::Active,
            ErrorCode::CompetitionNotActive
        );

        require!(
            clock.unix_timestamp < competition.end_time,
            ErrorCode::CompetitionEnded
        );

        require!(
            trading_account.owner == player,
            ErrorCode::UnauthorizedPlayer
        );

        require!(
            trading_account.active_positions < 10,
            ErrorCode::TooManyPositions
        );

        let cost = args.size;
        require!(
            trading_account.virtual_balance >= cost,
            ErrorCode::InsufficientBalance
        );

        trading_account.virtual_balance = trading_account.virtual_balance.checked_sub(cost).unwrap();
        trading_account.active_positions = trading_account.active_positions.checked_add(1).unwrap();

        position.owner = player;
        position.trading_account = trading_account.key();
        position.pair_index = args.pair_index;
        position.direction = args.direction;
        position.entry_price = args.current_price;
        position.size = args.size;
        position.take_profit = args.take_profit;
        position.stop_loss = args.stop_loss;
        position.opened_at = clock.unix_timestamp;
        position.is_active = true;

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
        pub trading_account: TradingAccount,
        pub position: Position,
    }

    #[arguments]
    pub struct Args {
        pub pair_index: u8,
        pub direction: PositionType,
        pub current_price: u64,
        pub size: u64,
        pub take_profit: Option<u64>,
        pub stop_loss: Option<u64>,
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Competition is not active")]
    CompetitionNotActive,
    #[msg("Competition has ended")]
    CompetitionEnded,
    #[msg("Unauthorized player")]
    UnauthorizedPlayer,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Too many active positions")]
    TooManyPositions,
}
