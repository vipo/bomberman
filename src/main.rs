use anyhow::anyhow;
use tide::{Error, Request, StatusCode};

mod game;
mod responses;
mod state;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = state::new(10240);
    let mut app = tide::with_state(state);
    app.at("/game/new/:name").post(new_game);
    app.at("/game/").get(list_games);
    //app.at("/game/:uuid").post(command);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn not_found() -> Error {
    Error::new(StatusCode::NotFound, anyhow!("Not found"))
}

async fn new_game(req: Request<state::State>) -> tide::Result {
    let name = req.param("name")?;
    let game = game::templates::by_name(name)
        .map(game::new)
        .ok_or(not_found())?;
    let uuid = req.state().insert_and_evict(game);
    responses::new_game_created(&uuid)
}

async fn list_games(req: Request<state::State>) -> tide::Result {
    let games = req.state().list_games();
    responses::list_games(&games)
}
