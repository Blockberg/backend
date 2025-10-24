use bolt_lang::*;

declare_id!("8NHfJVx1ZD8tnb23v4xvTsUdhMxhHbjYpPz4ZDstobYP");

#[component]
pub struct Position {
    pub owner: Pubkey,
    pub trading_account: Pubkey,
    pub pair_index: u8,
    pub direction: PositionType,
    pub entry_price: u64,
    pub size: u64,
    pub take_profit: Option<u64>,
    pub stop_loss: Option<u64>,
    pub opened_at: i64,
    pub is_active: bool,
}

#[component_deserialize]
#[derive(PartialEq)]
pub enum PositionType {
    Long,
    Short,
}

impl Default for Position {
    fn default() -> Self {
        Self::new(PositionInit {
            owner: Pubkey::default(),
            trading_account: Pubkey::default(),
            pair_index: 0,
            direction: PositionType::Long,
            entry_price: 0,
            size: 0,
            take_profit: None,
            stop_loss: None,
            opened_at: 0,
            is_active: false,
        })
    }
}
