use super::game::Game;
use chrono::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    games: Arc<RwLock<HashMap<Uuid, Game>>>,
    stack: Arc<RwLock<Vec<Uuid>>>,
    size: usize,
}

#[derive(Debug)]
pub struct ActiveGame {
    pub uuid: Uuid,
    pub started: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl State {
    pub fn insert_and_evict(&self, game: Game) -> Uuid {
        let uuid = Uuid::new_v4();
        let mut games = self.games.write().unwrap();
        games.insert(uuid, game);
        let mut stack = self.stack.write().unwrap();
        stack.push(uuid);
        let diff = if stack.len() >= self.size {
            stack.len() - self.size
        } else {
            0
        };
        for _ in 0..diff {
            games.remove(&stack.remove(0));
        }
        uuid
    }

    pub fn apply_to_game<F>(&self, uuid: Uuid, f: F)
    where
        F: Fn(&mut Game) -> (),
    {
        let mut games = self.games.write().unwrap();
        if let Some(game) = games.get_mut(&uuid) {
            f(game);
        }
    }

    pub fn list_games(&self) -> Vec<ActiveGame> {
        let stack = self.stack.write().unwrap();
        let mut uuids = stack.to_vec();
        uuids.reverse();
        let mut result = vec![];
        for uuid in uuids {
            let games = self.games.read().unwrap();
            let game = games.get(&uuid).unwrap();
            result.push(ActiveGame {
                uuid: uuid,
                started: game.started,
                updated: game.updated,
            });
        }
        result
    }
}

pub fn new(size: usize) -> State {
    State {
        games: Arc::new(RwLock::new(HashMap::new())),
        stack: Arc::new(RwLock::new(Vec::new())),
        size: size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uuids(gs: &Vec<ActiveGame>) -> Vec<Uuid> {
        gs.iter().map(|g| g.uuid).collect()
    }

    #[test]
    fn test_size() {
        let sut = new(2);
        assert_eq!(sut.list_games().len(), 0);
        let uuid_1 = sut.insert_and_evict(crate::game::new(crate::game::templates::SMALL_1));
        assert_eq!(uuids(&sut.list_games()), vec![uuid_1]);
        let uuid_2 = sut.insert_and_evict(crate::game::new(crate::game::templates::WIDE_1));
        assert_eq!(uuids(&sut.list_games()), vec![uuid_2, uuid_1]);
        let uuid_3 = sut.insert_and_evict(crate::game::new(crate::game::templates::WIDE_1));
        assert_eq!(uuids(&sut.list_games()), vec![uuid_3, uuid_2]);
    }
}
