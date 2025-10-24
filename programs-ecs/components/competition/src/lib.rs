use bolt_lang::*;

declare_id!("zQKpawEnbpdRj7MPzPuBKjJdgmSCC2A1aNi3NbGv4PN");

#[component]
pub struct Competition {
    pub round_id: u64,
    pub state: CompetitionState,
    pub admin: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub entry_fee: u64,
    pub prize_pool: u64,
    pub total_participants: u32,
    pub winner: Option<Pubkey>,
    pub treasury: Pubkey,
}

#[component_deserialize]
#[derive(PartialEq)]
pub enum CompetitionState {
    Pending,
    Active,
    Ended,
    Settled,
}

impl Default for Competition {
    fn default() -> Self {
        Self::new(CompetitionInit {
            round_id: 0,
            state: CompetitionState::Pending,
            admin: Pubkey::default(),
            start_time: 0,
            end_time: 0,
            entry_fee: 0,
            prize_pool: 0,
            total_participants: 0,
            winner: None,
            treasury: Pubkey::default(),
        })
    }
}
