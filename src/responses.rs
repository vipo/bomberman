use serde_json::value::*;
use tide::prelude::*;
use tide::{Body, Response, StatusCode};
use uuid::Uuid;

pub fn new_game_created(uuid: &Uuid) -> tide::Result {
    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&json!({"uuid": uuid.to_string()}))?)
        .build())
}

fn active_game_json(g: &crate::state::ActiveGame) -> Value {
    json!({
        "uuid": g.uuid.to_string(),
        "created": g.started.to_string(),
        "updated": g.updated.to_string()
    })
}

pub fn list_games(games: &Vec<crate::state::ActiveGame>) -> tide::Result {
    let entries: Vec<Value> = games.iter().map(active_game_json).collect();
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&json![entries])?)
        .build())
}

pub fn ok() -> tide::Result {
    Ok(Response::builder(StatusCode::Ok).build())
}
