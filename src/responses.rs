use serde_json::value::*;
use tide::prelude::*;
use tide::{Body, Response, StatusCode};
use uuid::Uuid;

#[derive(Debug)]
pub struct NewGame {
    pub uuid: Uuid,
    pub width: usize,
    pub height: usize,
}

pub fn new_game_created(new_game: &NewGame) -> tide::Result {
    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&json!({
                "uuid": new_game.uuid.to_string(),
                "height": new_game.height,
                "width": new_game.width}))?)
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

pub fn command(surrounding: &Option<crate::game::Surroundings>) -> tide::Result {
    match surrounding {
        None => Ok(Response::builder(StatusCode::Ok).build()),
        Some(s) => Ok(Response::builder(StatusCode::Ok)
            .body(Body::from_json(&json!({
                "surrounding": json!({
                    "bombermans": s.bombermans,
                    "ghosts": s.ghosts,
                    "wall": s.wall,
                    "bricks": s.bricks,
                    "gates": s.gates,
                })
            }))?)
            .build()),
    }
}
