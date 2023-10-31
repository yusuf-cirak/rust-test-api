use std::{sync::Arc, vec};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{Chat, ChatDto, GroupedChat},
    AppState,
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn get_all_chats_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(Chat, "SELECT * FROM chats")
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let chats = query_result.unwrap();

    let mut chat_groups: Vec<GroupedChat> = vec![];

    for chat in chats.iter() {
        let chat_dto = ChatDto {
            Id: chat.Id,
            Name: chat.Name.clone(),
            MatchId: chat.MatchId.clone(),
        };

        let mut is_exist = false;

        for chat_group in chat_groups.iter_mut() {
            if chat.LeagueId == chat_group.LeagueId {
                chat_group.Chats.push(chat_dto.clone());
                is_exist = true;
                break;
            }
        }

        if !is_exist {
            let group_chat_to_add = GroupedChat {
                LeagueId: chat.LeagueId.clone(),
                LeagueName: chat.LeagueName.clone(),
                Chats: vec![chat_dto],
            };

            chat_groups.push(group_chat_to_add);
        }
    }

    Ok(Json(chat_groups))
}
