use super::game::Game;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    games: Arc<DashMap<Uuid, Game>>,
}

const STATE_SIZE: usize = 10240;

impl State {
    pub fn insert(&self, game: Game) -> Uuid {
        let uuid = Uuid::new_v4();
        self.games.insert(uuid, game);
        uuid
    }
}

pub fn new() -> State {
    State {
        games: Arc::new(DashMap::new()),
    }
}
