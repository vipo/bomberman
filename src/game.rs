use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Cell {
    Empty,
    Wall,
    Brick,
    OpenGate,
    HiddenGate,
}

type Coord = (usize, usize);

#[derive(Debug)]
pub struct Game {
    width: usize,
    height: usize,
    landscape: HashMap<Coord, Cell>,
}

type Template = &'static [&'static str];

pub mod templates {
    use super::*;

    pub fn by_name(name: &str) -> Option<Template> {
        match name {
            "SMALL_1" => Some(SMALL_1),
            "WIDE_1" => Some(WIDE_1),
            _ => None,
        }
    }

    #[rustfmt::skip]
    pub const SMALL_1: Template = &[
        "XXXXXXXXXXXXXXX",
        "X     BBBBBBBBX",
        "XBXBX X X X X X",
        "X   B B       X",
        "X X X X X XBXBX",
        "X   B   B     X",
        "X X XBXBX XBXBX",
        "X         B B X",
        "XBXBX XBXBXBXBX",
        "X             X",
        "XBXBXBXBXBXBX X",
        "X             X",
        "X XBXBXBXBXBXBX",
        "X            GX",
        "XXXXXXXXXXXXXXX"];

    #[rustfmt::skip]
    pub const WIDE_1: Template = &[
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        "X     BBBBBBBBX                                           GX",
        "XBXBX X X X X X                                            X",
        "X   B B       X                                            X",
        "X X X X X XBXBX                                            X",
        "X   B   B     X                                            X",
        "X X XBXBX XBXBX                                            X",
        "X         B B X                                            X",
        "XBXBX XBXBXBXBX                                            X",
        "X             X                                            X",
        "XBXBXBXBXBXBX X                                            X",
        "X             X                                            X",
        "X XBXBXBXBXBXBX                                            X",
        "X                                                          X",
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"];

    pub fn cell_from_char(c: char) -> Option<Cell> {
        match c {
            ' ' => Some(Cell::Empty),
            'X' => Some(Cell::Wall),
            'B' => Some(Cell::Brick),
            'G' => Some(Cell::OpenGate),
            'H' => Some(Cell::HiddenGate),
            _ => None,
        }
    }
}

pub fn new(template: &[&str]) -> Game {
    let mut m: HashMap<Coord, Cell> = HashMap::new();
    let mut width = 0;
    for h in 0..template.len() {
        let row: &str = template[h];
        width = cmp::max(width, row.len());
        for (w, c) in row.chars().enumerate() {
            m.insert((h, w), templates::cell_from_char(c).unwrap());
        }
    }
    return Game {
        width: width,
        height: template.len(),
        landscape: m,
    };
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
        assert_eq!(sut.landscape.get(&(1, 58)).unwrap(), &Cell::OpenGate);
    }
}
