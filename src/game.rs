use chrono::prelude::*;
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
}

impl Game {
    pub fn bomberman_up(&mut self) {
        self.mv((-1, 0));
        self.updated = Utc::now();
    }
    pub fn bomberman_down(&mut self) {
        self.mv((1, 0));
        self.updated = Utc::now();
    }
    pub fn bomberman_left(&mut self) {
        self.mv((0, -1));
        self.updated = Utc::now();
    }
    pub fn bomberman_right(&mut self) {
        self.mv((1, 0));
        self.updated = Utc::now();
    }
    fn mv(&mut self, offset: (i8, i8)) {
        let new = Game::add(self.bomberman, offset);
        if let Some(c) = self.landscape.get(&new) {
            match c {
                Cell::Empty => self.bomberman = new,
                _ => {}
            }
        }
    }
    fn add(this: (usize, usize), that: (i8, i8)) -> (i8, i8) {
        (this.0 + that.0, this.1 + that.1)
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
                templates::LandscapeFromChar::Bomber => bomber = Some((h, w)),
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
        assert_eq!(sut.landscape.get(&(1, 1)).unwrap(), &Cell::Bomber);
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
        assert_eq!(sut.landscape.get(&(1, 1)).unwrap(), &Cell::Bomber);
        assert_eq!(sut.landscape.get(&(1, 58)).unwrap(), &Cell::OpenGate);
    }
}
