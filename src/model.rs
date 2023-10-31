use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Chat {
    pub Id: i32,
    pub Name: String,
    pub LeagueName: String,
    pub LeagueId: String,
    pub MatchId: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatDto {
    pub Id: i32,
    pub Name: String,
    pub MatchId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupedChat {
    pub LeagueId: String,
    pub LeagueName: String,
    pub Chats: Vec<ChatDto>,
}
