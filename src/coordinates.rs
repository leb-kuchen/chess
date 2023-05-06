use crate::*;

lazy_static! {
    pub static ref VALID_COORDINATES: Set<ChessSquareCoordinates> = set_valid_squares();
}

fn set_valid_squares() -> Set<ChessSquareCoordinates> {
    let mut valid_squares = Set::new();
    for row in 1..=8 {
        for col in 'A'..='H' {
            valid_squares.insert(ChessSquareCoordinates::new(col, row));
        }
    }
    valid_squares
}
impl ChessSquareCoordinates {
    pub fn new(col: char, row: u8) -> Self {
        Self { row, col }
    }
}
macro_rules! impl_squares_iter {
    ($square_dir_iter: ty,  $dir: expr) => {
        impl Iterator for $square_dir_iter {
            type Item = ChessSquareCoordinates;
            fn next(&mut self) -> Option<Self::Item> {
                match $dir {
                    CoordinateDirection::Row(step) => self.0.row.shift(step),
                    CoordinateDirection::Col(step) => self.0.col.shift(step),
                    CoordinateDirection::Diagonal((col_step, row_step)) => {
                        self.0.col.shift(col_step);
                        self.0.row.shift(row_step);
                    }
                };
                let new_coordinate = ChessSquareCoordinates::new(self.0.col, self.0.row);
                if !VALID_COORDINATES.contains(&new_coordinate) {
                    return None;
                }
                Some(new_coordinate)
            }
        }
    };
}
impl fmt::Display for ChessSquareCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.col, self.row)
    }
}
impl FromStr for ChessSquareCoordinates {
    type Err = ParseCoordinatesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_two_chars = s.trim().replace(' ', "").to_uppercase();
        let mut first_two_chars = first_two_chars.chars().take(2);
        let (col, row) = match (first_two_chars.next(), first_two_chars.next()) {
            (Some(col @ 'A'..='H'), Some(row @ '1'..='8')) => (col, row),
            _ => return Err(ParseCoordinatesError),
        };
        Ok(Self {
            col: col,
            row: row.to_digit(10).ok_or(ParseCoordinatesError)? as u8,
        })
    }
}
impl Ord for ChessSquareCoordinates {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        /*
        let rev_col_coordinates = |coordinates: &ChessSquareCoordinates|ChessSquareCoordinates {
            col: -(coordinates.col as u8 as i8 - 63 - 8) as u8 as char,
            ..*coordinates
        };
        let this_coordinates = rev_col_coordinates(&self);
        let other_coordinates = rev_col_coordinates(&other);
        other_coordinates.cmp(&this_coordinates)
        */
        // self.row.cmp(&other.row)
        todo!()
    }
}

trait ShiftCoordinates {
    fn shift(&mut self, step: i8);
}
impl ShiftCoordinates for u8 {
    fn shift(&mut self, step: i8) {
        //	println!("{} {}", self, step);
        *self = (*self as i8 + step) as u8;
    }
}
impl ShiftCoordinates for char {
    fn shift(&mut self, step: i8) {
        //	println!("{} {}", self, step);
        *self = (*self as u8 as i8 + step) as u8 as char;
    }
}
enum CoordinateDirection {
    Row(i8),
    Col(i8),
    Diagonal((i8, i8)),
}

// better names
// improve field with macro
// first test for easy development
// shift function
// coordinates string fromstr
//
impl_squares_iter! {UpperSquare, CoordinateDirection::Row(1)}
impl_squares_iter! {LeftSquare, CoordinateDirection::Col(-1)}
impl_squares_iter! {RightSquare, CoordinateDirection::Col(1)}
impl_squares_iter! {LowerSquare, CoordinateDirection::Row(-1)}
impl_squares_iter! {LowerRightSquare, CoordinateDirection::Diagonal((1, -1))}
impl_squares_iter! {LowerLeftSquare, CoordinateDirection::Diagonal((-1, -1))}
impl_squares_iter! {UpperRightSquare, CoordinateDirection::Diagonal((1, 1))}
impl_squares_iter! {UpperLeftSquare, CoordinateDirection::Diagonal((-1, 1))}
#[cfg(test)]
mod tests {
    use super::*;
    type Coords = ChessSquareCoordinates;
    #[test]
    fn check_upper() {
        let upper_d: Coords = Coords::from_str("D8").unwrap();
        let middle: Coords = Coords::from_str("D4").unwrap();
        assert_eq!(Some(upper_d), UpperSquare(middle).last());
    }
    #[test]
    fn check_left() {
        let top_d: Coords = Coords::from_str("A4").unwrap();
        let middle: Coords = Coords::from_str("D4").unwrap();
        assert_eq!(Some(top_d), LeftSquare(middle).last());
    }
    #[test]
    fn check_right() {
        let right_d: Coords = Coords::from_str("H4").unwrap();
        let middle: Coords = Coords::from_str("D4").unwrap();
        assert_eq!(Some(right_d), RightSquare(middle).last());
    }
    #[test]
    fn check_lower() {
        let lower_d: Coords = Coords::from_str("D1").unwrap();
        let middle: Coords = Coords::from_str("D4").unwrap();
        assert_eq!(Some(lower_d), LowerSquare(middle).last());
    }
    #[test]
    fn check_lower_right() {
        let lower_g: Coords = Coords::from_str("G1").unwrap();
        let down: Coords = Coords::from_str("D4").unwrap();
        let left = Coords::from_str("G4").unwrap();
        let lower_h = Coords::from_str("H3").unwrap();
        assert_eq!(Some(lower_g), LowerRightSquare(down).last());
        assert_eq!(Some(lower_h), LowerRightSquare(left).last());
    }
    #[test]
    fn check_lower_left() {
        let left_a: Coords = Coords::from_str("A3").unwrap();
        let down: Coords = Coords::from_str("E4").unwrap();
        let left = Coords::from_str("B4").unwrap();
        let lower_b = Coords::from_str("B1").unwrap();
        assert_eq!(Some(lower_b), LowerLeftSquare(down).last());
        assert_eq!(Some(left_a), LowerLeftSquare(left).last());
    }
    #[test]
    fn check_upper_right() {
        let right_h: Coords = Coords::from_str("H6").unwrap();
        let up: Coords = Coords::from_str("D5").unwrap();
        let left = Coords::from_str("G5").unwrap();
        let up_g = Coords::from_str("G8").unwrap();
        assert_eq!(Some(up_g), UpperRightSquare(up).last());
        assert_eq!(Some(right_h), UpperRightSquare(left).last());
    }
    #[test]
    fn check_upper_left() {
        let left_a: Coords = Coords::from_str("A5").unwrap();
        let up: Coords = Coords::from_str("E5").unwrap();
        let left = Coords::from_str("B4").unwrap();
        let upper_b = Coords::from_str("B8").unwrap();
        assert_eq!(Some(upper_b), UpperLeftSquare(up).last());
        assert_eq!(Some(left_a), UpperLeftSquare(left).last());
    }
}
//
//vec lice macro
pub struct LowerRightSquare(pub ChessSquareCoordinates);
pub struct LowerLeftSquare(pub ChessSquareCoordinates);
pub struct UpperRightSquare(pub ChessSquareCoordinates);
pub struct UpperLeftSquare(pub ChessSquareCoordinates);
//
pub struct RightSquare(pub ChessSquareCoordinates);
pub struct LeftSquare(pub ChessSquareCoordinates);
pub struct UpperSquare(pub ChessSquareCoordinates);
pub struct LowerSquare(pub ChessSquareCoordinates);
