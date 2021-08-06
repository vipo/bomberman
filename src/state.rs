use super::game::Game;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    games: Arc<DashMap<Uuid, Game>>,
}

pub fn new() -> State {
    return State {
        games: Arc::new(DashMap::new()),
    };
}
