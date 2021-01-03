// // TODO handle parsing of multi digit numbers
// fn to_map_coordinates(chess_coords: &str) -> (i32, i32) {
//     let row: Vec<&str> = chess_coords
//         .matches(char::is_numeric)
//         .collect()
//         .nth(0)
//         .parse::<i32>()
//         .unwrap();
//     let col: Vec<&str> = chess_coords.matches(char::is_alphabetic).collect();
//     println!("split: {:?} {:?}", v1, v2);
//     (1, 1)
// }

// #[test]
// fn test_to_map_coordinates() {
//     assert_eq!(to_map_coordinates("ZZ1"), (701, 0));
//     assert_eq!(to_map_coordinates("ZZ11"), (701, 10));
//     assert_eq!(to_map_coordinates("A1"), (0, 0));
//     assert_eq!(to_map_coordinates("A5"), (0, 4));
//     assert_eq!(to_map_coordinates("B2"), (1, 1));
//     assert_eq!(to_map_coordinates("Z1"), (25, 0));
//     assert_eq!(to_map_coordinates("AA1"), (26, 0));
//     assert_eq!(to_map_coordinates("AB1"), (27, 0));
//     assert_eq!(to_map_coordinates("YZ1"), (675, 0));
//     assert_eq!(to_map_coordinates("ZA1"), (676, 0));
//     assert_eq!(to_map_coordinates("ZB1"), (677, 0));
// }

pub fn from_map_coordinates(column: i32, row: i32) -> String {
    if column >= (27 * 26) {
        panic!("parameter column out of range [0..{}): {}", 27 * 26, column);
    }
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let first_c: usize = (column / 26) as usize;
    let second_c: usize = (column % 26) as usize;

    let s: String = format!(
        "{}{}{}",
        if column >= 26 {
            alphabet[first_c - 1].into()
        } else {
            ' '
        },
        alphabet[second_c],
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
