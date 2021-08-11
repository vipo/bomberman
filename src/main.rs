use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tide::{Error, Request, StatusCode};
use uuid::Uuid;

mod game;
mod responses;
mod state;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = state::new(1024);
    let mut app = tide::with_state(state);
    app.at("/game/new/:name").post(new_game);
    app.at("/game/").get(list_games);
    app.at("/game/:uuid").post(command);
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

#[derive(Deserialize, Serialize, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "name")]
enum Commands {
    MoveBomberman { direction: Direction },
    FetchMapArea { x: u8, y: u8 },
}

#[derive(Deserialize, Serialize)]
struct Command {
    command: Commands,
    additional: Option<Box<Command>>,
}

fn flatten<'a>(command: &'a Command) -> Vec<&'a Commands> {
    let mut result = vec![];
    let mut current: &Command = command;
    loop {
        result.push(&current.command);
        match &current.additional {
            None => {
                break;
            }
            Some(v) => {
                current = &v;
                continue;
            }
        };
    }
    result
}

async fn command(mut req: Request<state::State>) -> tide::Result {
    let uuid = req.param("uuid")?;
    let uuid = Uuid::parse_str(uuid)?;
    let command: Command = req.body_json().await?;
    let state = req.state();
    for command in flatten(&command) {
        match command {
            Commands::MoveBomberman { direction } => match direction {
                Direction::Up => state.apply_to_game(uuid, |g| g.bomberman_up()),
                Direction::Down => state.apply_to_game(uuid, |g| g.bomberman_down()),
                Direction::Left => state.apply_to_game(uuid, |g| g.bomberman_left()),
                Direction::Right => state.apply_to_game(uuid, |g| g.bomberman_right()),
            },
            Commands::FetchMapArea { x, y } => {}
        }
    }
    responses::ok()
}
