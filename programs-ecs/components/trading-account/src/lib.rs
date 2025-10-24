use bolt_lang::*;

declare_id!("3UhnNbUpRi1QM6szPYJce4tBNLCbjxMESJJ8touBd55h");

#[component]
pub struct TradingAccount {
    pub owner: Pubkey,
    pub competition: Pubkey,
    pub virtual_balance: u64,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub active_positions: u8,
}

impl Default for TradingAccount {
    fn default() -> Self {
        Self::new(TradingAccountInit {
            owner: Pubkey::default(),
            competition: Pubkey::default(),
            virtual_balance: 0,
            realized_pnl: 0,
            unrealized_pnl: 0,
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            active_positions: 0,
        })
    }
}
