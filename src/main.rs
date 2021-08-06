use tide::prelude::*;
use tide::Request;

mod game;
mod state;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = state::new();
    let mut app = tide::with_state(state);
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    let _game = game::new(&game::templates::SMALL_1);
    Ok(())
}

async fn order_shoes(mut req: Request<state::State>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes\n", name, legs).into())
}
