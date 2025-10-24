use bolt_lang::*;

declare_id!("5ohgmFUcN41uoZuP1QnFP9ErjDDCXA1FaxpFZAzfwU6q");

#[component]
pub struct Leaderboard {
    pub competition: Pubkey,
    pub entries: Vec<LeaderboardEntry>,
    pub last_updated: i64,
}

#[component_deserialize]
#[derive(Clone)]
pub struct LeaderboardEntry {
    pub player: Pubkey,
    pub pnl: i64,
    pub rank: u32,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::new(LeaderboardInit {
            competition: Pubkey::default(),
            entries: Vec::new(),
            last_updated: 0,
        })
    }
}
