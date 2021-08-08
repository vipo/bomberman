use tide::prelude::*;
use tide::{Body, Error, Request, Response, StatusCode};

use anyhow::anyhow;

mod game;
mod state;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = state::new(10240);
    let mut app = tide::with_state(state);
    app.at("/game/new/:name").post(new_game);
    app.at("/game/").get(list_games);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn not_found() -> Error {
    Error::new(StatusCode::NotFound, anyhow!("Not found"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    uuid: String,
}

async fn new_game(req: Request<state::State>) -> tide::Result {
    let name = req.param("name")?;
    let game = game::templates::by_name(name)
        .map(game::new)
        .ok_or(not_found())?;
    let uuid = req.state().insert_and_evict(game);
    let response = Game {
        uuid: uuid.to_string(),
    };
    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&response)?)
        .build())
}

async fn list_games(req: Request<state::State>) -> tide::Result {
    let games = req.state().list_games();
    let response: Vec<Game> = games
        .into_iter()
        .map(|u| Game {
            uuid: u.to_string(),
        })
        .collect();
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&response)?)
        .build())
}
