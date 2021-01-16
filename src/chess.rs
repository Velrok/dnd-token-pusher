use regex::Regex;
use std::num::ParseIntError;
use std::option::NoneError;

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const BASE: i32 = 26;

#[derive(Debug)]
struct ParseError {
    err: String,
}

impl From<NoneError> for ParseError {
    fn from(e: NoneError) -> Self {
        ParseError {
            err: "No match found.".into(),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError {
            err: format!("{}", e),
        }
    }
}

fn to_map_coordinates(chess_coords: &str) -> Result<(i32, i32), ParseError> {
    lazy_static! {
        static ref CHESS_NOTATION_PARTS: Regex = Regex::new(r"^([A-Z]+)(\d+)$").unwrap();
    }
    let caps = CHESS_NOTATION_PARTS.captures(chess_coords)?;
    let row: i32 = caps.get(2)?.as_str().parse()?;
    let col: i32 = caps
        .get(1)?
        .as_str()
        .chars()
        .rev()
        .enumerate()
        .fold(0 as i32, |agg, (i, c)| {
            let p: i32 = ALPHABET.iter().position(|&x| x == c).unwrap() as i32 + 1;
            agg + BASE.pow(i as u32) * p
        });

    Ok((col - 1, row - 1)) // back to starting at 0
}

#[test]
fn test_to_map_coordinates() {
    assert_eq!(to_map_coordinates("ZZ1").ok(), Some((701, 0)));
    assert_eq!(to_map_coordinates("ZZ11").ok(), Some((701, 10)));
    assert_eq!(to_map_coordinates("A1").ok(), Some((0, 0)));
    assert_eq!(to_map_coordinates("A5").ok(), Some((0, 4)));
    assert_eq!(to_map_coordinates("B2").ok(), Some((1, 1)));
    assert_eq!(to_map_coordinates("Z1").ok(), Some((25, 0)));
    assert_eq!(to_map_coordinates("AA1").ok(), Some((26, 0)));
    assert_eq!(to_map_coordinates("AB1").ok(), Some((27, 0)));
    assert_eq!(to_map_coordinates("YZ1").ok(), Some((675, 0)));
    assert_eq!(to_map_coordinates("ZA1").ok(), Some((676, 0)));
    assert_eq!(to_map_coordinates("ZB1").ok(), Some((677, 0)));
}

// TODO: make prive so we have to use the public enums!
pub fn from_map_coordinates(column: i32, row: i32) -> String {
    if column >= (27 * BASE) {
        panic!(
            "parameter column out of range [0..{}): {}",
            27 * BASE,
            column
        );
    }
    let first_c: usize = (column / BASE) as usize;
    let second_c: usize = (column % BASE) as usize;

    let s: String = format!(
        "{}{}{}",
        if column >= BASE {
            ALPHABET[first_c - 1].into()
        } else {
            ' '
        },
        ALPHABET[second_c],
        row + 1,
    );
    s.trim().to_string()
}

#[test]
fn test_chess_coordinates() {
    assert_eq!("A1", from_map_coordinates(0, 0));
    assert_eq!("A5", from_map_coordinates(0, 4));
    assert_eq!("B2", from_map_coordinates(1, 1));
    assert_eq!("Z1", from_map_coordinates(25, 0));
    assert_eq!("AA1", from_map_coordinates(26, 0));
    assert_eq!("AB1", from_map_coordinates(27, 0));
    assert_eq!("YZ1", from_map_coordinates(675, 0));
    assert_eq!("ZA1", from_map_coordinates(676, 0));
    assert_eq!("ZB1", from_map_coordinates(677, 0));
    assert_eq!("ZZ1", from_map_coordinates(701, 0));
    // from_map_coordinates(702, 0); // panic!
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Coordinates {
    Chess(String),
    Map((i32, i32)),
}

impl Coordinates {
    pub fn to_map(&self) -> Result<Coordinates, ParseError> {
        Ok(match self {
            Coordinates::Map(x) => Coordinates::Map(*x),
            Coordinates::Chess(s) => Coordinates::Map(to_map_coordinates(s.as_str())?),
        })
    }

    pub fn to_chess(&self) -> Coordinates {
        match self {
            Coordinates::Chess(s) => Coordinates::Chess((*s).to_owned()),
            Coordinates::Map((col, row)) => Coordinates::Chess(from_map_coordinates(*col, *row)),
        }
    }
}

#[test]
fn test_coordinates_conversion() {
    assert_eq!(
        Coordinates::Chess("A1".to_string()),
        Coordinates::Chess("A1".to_string())
    );
    assert_eq!(
        Coordinates::Chess("A1".to_string()),
        Coordinates::Chess("A1".to_string()).to_chess()
    );
    assert_eq!(
        Some(Coordinates::Map((0, 0))),
        Coordinates::Chess("A1".to_string()).to_map().ok()
    );

    assert_eq!(Coordinates::Map((0, 0)), Coordinates::Map((0, 0)));
    assert_eq!(
        Some(Coordinates::Map((0, 0))),
        Coordinates::Map((0, 0)).to_map().ok()
    );
    assert_eq!(
        Coordinates::Chess("A1".to_string()),
        Coordinates::Map((0, 0)).to_chess()
    );
}
