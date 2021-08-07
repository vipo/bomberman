use tide::prelude::*;
use tide::{Body, Error, Request, Response, StatusCode};

use anyhow::anyhow;

mod game;
mod state;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = state::new();
    let mut app = tide::with_state(state);
    app.at("/game/new/:name").post(new_game);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn not_found() -> Error {
    Error::new(StatusCode::NotFound, anyhow!("Not found"))
}

#[derive(Debug, Serialize, Deserialize)]
struct NewGame {
    uuid: String,
}

async fn new_game(req: Request<state::State>) -> tide::Result {
    let name = req.param("name")?;
    let game = game::templates::by_name(name)
        .map(game::new)
        .ok_or(not_found())?;
    let uuid = req.state().insert(game);
    let response = NewGame {
        uuid: uuid.to_string(),
    };
    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&response)?)
        .build())
}
