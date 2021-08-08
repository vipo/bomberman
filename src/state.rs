use super::game::Game;
use chashmap::CHashMap;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    games: Arc<CHashMap<Uuid, Game>>,
    stack: Arc<Mutex<Vec<Uuid>>>,
    size: usize,
}

impl State {
    pub fn insert_and_evict(&self, game: Game) -> Uuid {
        let uuid = Uuid::new_v4();
        self.games.insert(uuid, game);
        let mut stack = self.stack.lock().unwrap();
        stack.push(uuid);
        let diff = if stack.len() >= self.size {
            stack.len() - self.size
        } else {
            0
        };
        for _ in 0..diff {
            self.games.remove(&stack.remove(0));
        }
        uuid
    }

    pub fn list_games(&self) -> Vec<Uuid> {
        let stack = self.stack.lock().unwrap();
        let mut result = stack.to_vec();
        result.reverse();
        result
    }
}

pub fn new(size: usize) -> State {
    State {
        games: Arc::new(CHashMap::new()),
        stack: Arc::new(Mutex::new(Vec::new())),
        size: size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let sut = new(2);
        assert_eq!(sut.list_games().len(), 0);
        let uuid_1 = sut.insert_and_evict(crate::game::new(crate::game::templates::SMALL_1));
        assert_eq!(sut.list_games(), vec![uuid_1]);
        let uuid_2 = sut.insert_and_evict(crate::game::new(crate::game::templates::WIDE_1));
        assert_eq!(sut.list_games(), vec![uuid_2, uuid_1]);
        let uuid_3 = sut.insert_and_evict(crate::game::new(crate::game::templates::WIDE_1));
        assert_eq!(sut.list_games(), vec![uuid_3, uuid_2]);
    }
}
