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

#[derive(Debug, Deserialize, Serialize)]
pub struct List {
    pub head: Option<(usize, usize)>,
    pub tail: Option<Box<List>>,
}

fn to_list(vs: &Vec<(usize, usize)>) -> List {
    let mut result = List {
        head: None,
        tail: None,
    };
    for v in vs {
        result = List {
            head: Some((v.0, v.1)),
            tail: Some(Box::new(result)),
        }
    }
    result
}

pub fn command2(surrounding: &Option<crate::game::Surroundings>, bomb: &Option<crate::game::BombStatus>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&json!({
            "surrounding": surrounding.as_ref().map(|v| surr_json(v)),
            "bomb": bomb.as_ref().map(|v| json!(v.coords)),
        }))?).build())
}

fn surr_json(s: &crate::game::Surroundings) -> serde_json::Value {
    json!({
        "bombermans": to_list(&s.bombermans),
        "ghosts": to_list(&s.ghosts),
        "wall": to_list(&s.wall),
        "bricks": to_list(&s.bricks),
        "gates": to_list(&s.gates),
    })
}