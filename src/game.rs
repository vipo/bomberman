use chrono::prelude::*;
use rand::prelude::*;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Brick,
    OpenGate,
    HiddenGate,
    Ghost,
}

type Coord = (usize, usize);

#[derive(Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    landscape: HashMap<Coord, Cell>,
    pub started: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    bomberman: (usize, usize),
    active: bool,
    surrounding_size: usize,
    bomb: Option<(DateTime<Utc>, (usize, usize))>,
}

pub struct Surroundings {
    pub bricks: Vec<(usize, usize)>,
    pub wall: Vec<(usize, usize)>,
    pub bombermans: Vec<(usize, usize)>,
    pub ghosts: Vec<(usize, usize)>,
    pub gates: Vec<(usize, usize)>,
}

pub struct BombStatus {
    pub coords: (usize, usize)
}

impl Game {
    pub fn bomb_status(&self) -> Option<BombStatus> {
        self.bomb.map(|b| BombStatus {coords: b.1})
    }
    pub fn surrounding(&self) -> Surroundings {
        let h_min = cmp::max(0, self.bomberman.0 as i8 - self.surrounding_size as i8) as usize;
        let h_max = cmp::min(
            self.height as i8 - 1,
            self.bomberman.0 as i8 + self.surrounding_size as i8,
        ) as usize;
        let w_min = cmp::max(0, self.bomberman.1 as i8 - self.surrounding_size as i8) as usize;
        let w_max = cmp::min(
            self.width as i8 - 1,
            self.bomberman.1 as i8 + self.surrounding_size as i8,
        ) as usize;
        let mut result = Surroundings {
            bricks: vec![],
            wall: vec![],
            bombermans: vec![self.bomberman],
            ghosts: vec![],
            gates: vec![],
        };
        for h in h_min..=h_max {
            for w in w_min..=w_max {
                let coord = (h, w);
                match self.landscape.get(&coord) {
                    Some(Cell::Brick) => result.bricks.push(coord),
                    Some(Cell::Wall) => result.wall.push(coord),
                    Some(Cell::OpenGate) => result.gates.push(coord),
                    Some(Cell::Ghost) => result.ghosts.push(coord),
                    _ => {}
                }
            }
        }
        result
    }

    pub fn bomberman_up(&mut self) {
        self.mv((-1, 0));
        self.blast();
    }
    pub fn bomberman_down(&mut self) {
        self.mv((1, 0));
        self.blast();
    }
    pub fn bomberman_left(&mut self) {
        self.mv((0, -1));
        self.blast();
    }
    pub fn bomberman_right(&mut self) {
        self.mv((0, 1));
        self.blast();
    }
    fn mv(&mut self, offset: (i8, i8)) {
        let now = Utc::now().timestamp();
        if now > self.updated.timestamp() && self.active {
            let new = Game::add(self.bomberman, offset);
            if let Some(c) = self.landscape.get(&new) {
                match c {
                    Cell::Empty => {
                        self.bomberman = new;
                        self.updated = Utc::now();
                    }
                    Cell::OpenGate | Cell::Ghost => {
                        self.bomberman = new;
                        self.updated = Utc::now();
                        self.active = false;
                    }
                    _ => {}
                }
            }
        };
    }
    pub fn plant_bomb(&mut self) {
        let now = Utc::now();
        self.blast();
        match self.bomb {
            None => {self.bomb = Some((now, self.bomberman))}
            Some(_) => () 
        }
    }
    fn blast(&mut self) {
        let now = Utc::now();
        match self.bomb {
            None => {}
            Some((planted, coords)) => {
                if now.timestamp() - planted.timestamp() >=4 {
                    self.demolish(Game::add(coords, (0, 1)));
                    self.demolish(Game::add(coords, (1, 1)));
                    self.demolish(Game::add(coords, (0, -1)));
                    self.demolish(Game::add(coords, (-1, -1)));
                    self.bomb = None;
                }
            }
        }
    }
    fn demolish(&mut self, coords: (usize, usize)) {
        if let Some(c) = self.landscape.get_mut(&coords) {
            match c {
                Cell::Brick | Cell::Ghost => {
                    *c = Cell::Empty;
                    self.updated = Utc::now();
                }
                _ => {}
            }
        }
    }
    fn add(this: (usize, usize), that: (i8, i8)) -> (usize, usize) {
        (
            (this.0 as i8 + that.0) as usize,
            (this.1 as i8 + that.1) as usize,
        )
    }
}

type Template = &'static [&'static str];

pub mod templates {
    use super::*;

    pub fn by_name(name: &str) -> Option<Template> {
        match name.to_lowercase().as_str() {
            "small_1" => Some(SMALL_1),
            "wide_1" => Some(WIDE_1),
            _ => None,
        }
    }

    pub fn random() -> Template {
        let mut rng = rand::thread_rng();
        let mut ts: Vec<Template> = vec![SMALL_1, WIDE_1];
        ts.shuffle(&mut rng);
        ts.get(0).unwrap()
    }

    #[rustfmt::skip]
    pub const SMALL_1: Template = &[
        "XXXXXXXXXXXXXXX",
        "XM    BBBBBBBBX",
        "XBXBX X X X X X",
        "X   B B   B  GX",
        "X X X X X XBXBX",
        "X   B   B     X",
        "X X XBXBX XBXBX",
        "X         B B X",
        "XBXBX XBXBXBXBX",
        "X     BG      X",
        "XBXBX XBXBXBX X",
        "X     B       X",
        "X XBXBXBXBXBXBX",
        "X            OX",
        "XXXXXXXXXXXXXXX"];

    #[rustfmt::skip]
    pub const WIDE_1: Template = &[
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        "XM    BBBBBBBBX                                           OX",
        "XBXBX X X X X X                                            X",
        "X   B B       X                                            X",
        "X X X X X XBXBX                                            X",
        "X   B   B     X                                            X",
        "X X XBXBX XBXBX          BBBBBBBBBBBBBB                    X",
        "X         B B X          B      G     B                    X",
        "XBXBX XBXBXBXBX          BBBBBBBBBBBBBB                    X",
        "X             X                                            X",
        "XBXBXBXBXBXBX X                                            X",
        "X             X                                            X",
        "X XBXBXBXBXBXBX                                            X",
        "X                                                          X",
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"];

    pub enum LandscapeFromChar {
        Land { cell: Cell },
        Bomber,
        Unknown,
    }

    pub fn cell_from_char(c: char) -> LandscapeFromChar {
        match c {
            ' ' => LandscapeFromChar::Land { cell: Cell::Empty },
            'X' => LandscapeFromChar::Land { cell: Cell::Wall },
            'B' => LandscapeFromChar::Land { cell: Cell::Brick },
            'O' => LandscapeFromChar::Land {
                cell: Cell::OpenGate,
            },
            'H' => LandscapeFromChar::Land {
                cell: Cell::HiddenGate,
            },
            'M' => LandscapeFromChar::Bomber,
            'G' => LandscapeFromChar::Land { cell: Cell::Ghost },
            _ => LandscapeFromChar::Unknown,
        }
    }
}

pub fn new(template: &[&str]) -> Game {
    let mut m: HashMap<Coord, Cell> = HashMap::new();
    let mut width = 0;
    let mut bomber = None;
    for h in 0..template.len() {
        let row: &str = template[h];
        width = cmp::max(width, row.len());
        for (w, c) in row.chars().enumerate() {
            match templates::cell_from_char(c) {
                templates::LandscapeFromChar::Land { cell } => {
                    m.insert((h, w), cell);
                }
                templates::LandscapeFromChar::Bomber => {
                    bomber = Some((h, w));
                    m.insert((h, w), Cell::Empty);
                }
                templates::LandscapeFromChar::Unknown => panic!("Unknown char in template {}", c),
            }
        }
    }
    Game {
        width: width,
        height: template.len(),
        landscape: m,
        started: Utc::now(),
        updated: Utc::now(),
        bomberman: bomber.unwrap(),
        active: true,
        surrounding_size: 7,
        bomb: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_1() {
        let sut = new(templates::SMALL_1);
        assert_eq!(sut.height, 15);
        assert_eq!(sut.width, 15);
        assert_eq!(sut.landscape.len(), 15 * 15);
        assert_eq!(sut.bomberman, (1, 1));
        assert_eq!(sut.landscape.get(&(3, 13)).unwrap(), &Cell::Ghost);
        assert_eq!(sut.landscape.get(&(0, 13)).unwrap(), &Cell::Wall);
        assert_eq!(sut.landscape.get(&(1, 13)).unwrap(), &Cell::Brick);
        assert_eq!(sut.landscape.get(&(13, 13)).unwrap(), &Cell::OpenGate);
    }

    #[test]
    fn test_wide_1() {
        let sut = new(templates::WIDE_1);
        assert_eq!(sut.height, 15);
        assert_eq!(sut.width, 60);
        assert_eq!(sut.landscape.len(), 15 * 60);
        assert_eq!(sut.bomberman, (1, 1));
        assert_eq!(sut.landscape.get(&(1, 58)).unwrap(), &Cell::OpenGate);
    }
}
