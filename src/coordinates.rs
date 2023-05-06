use crate::*;

macro_rules! impl_squares_iter {													// optional
    ($square_dir: ty,  $row_or_col: expr, $max_or_min: expr, $op: tt, $dir: ident, $dir2: ident, $op2: tt) => {
        impl Iterator for $square_dir {
            type Item = ChessSquareCoordinates;
            fn next(&mut self) -> Option<Self::Item> {
                match $row_or_col {
                	// ident2 ?
                    RowOrCol::Row(step) => self.0.row.shift(step),
                    RowOrCol::Col(step) => self.0.col.shift(step),
                    RowOrCol::Both((col_step, row_step)) => {
                    	self.0.col.shift(col_step);
                    	self.0.row.shift(row_step);
                    }
                };
                match $row_or_col {
                    RowOrCol::Col(_) | RowOrCol::Row(_) => {
                    	if let MaxOrMin::MinOrMax(border) = $max_or_min {
                			if compare!(self.0.$dir $op border) {
                        		return None;
                        	}
                		}
                    }

                    RowOrCol::Both(_) => match $max_or_min {
                        MaxOrMin::MinAndMax((border1, border2)) => {
                        	if compare!(self.0.$dir $op border1) || compare!(self.0.$dir2 $op2 border2) {
                        		return None
                        	}
                        }
                        _ => (),
                    },
                }
                // stop when self in map is enemy
                Some(ChessSquareCoordinates {
                    row: self.0.row,
                    col: self.0.col,
                })
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
// impl PartialOrd ?
// remove  derive
macro_rules! compare {
    ($op: expr) => {
        $op
    };
}

enum TypeToShift {
    Int,
    Char,
}

trait ShiftCoordinates {
    fn shift(&mut self, step: i8);
}
// macro shift Coordiantes => Same Type
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
enum MaxOrMin<T, U> {
    MinOrMax(T),
    MinAndMax((T, U)),
    /*
    Max((u8, Option<u8>)),
    Min((u8, Option<u8>)),
    MinMax((u8, u8)),
    MaxMin((u8, u8)),
    */
}

enum RowOrCol {
    Row(i8),
    Col(i8),
    Both((i8, i8)),
}
enum ToCheck {
    Either,
    Both,
}

// better names
// improve field with macro
// first test for easy development
// shift function
// coordinates string fromstr
//																	ignore second generic if same type || more intiutive ordering
impl_squares_iter! {UpperSquare, RowOrCol::Row(1), MaxOrMin::MinOrMax::<u8, u8>(8), >, row, row, >}
impl_squares_iter! {LeftSquare, RowOrCol::Col(-1), MaxOrMin::MinOrMax::<char, char>('A'), <, col, col, <}
impl_squares_iter! {RightSquare, RowOrCol::Col(1), MaxOrMin::MinOrMax::<char, char>('H'), >, col, col, >}
impl_squares_iter! {LowerSquare, RowOrCol::Row(-1), MaxOrMin::MinOrMax::<u8, u8>(1), <, row, row, <}
impl_squares_iter! {LowerRightSquare, RowOrCol::Both((1, -1)), MaxOrMin::MinAndMax::<u8, char>((1, 'H')), <, row, col, >}
impl_squares_iter! {LowerLeftSquare, RowOrCol::Both((-1, -1)), MaxOrMin::MinAndMax::<u8, char>((1, 'A')), <, row, col, <}
impl_squares_iter! {UpperRightSquare, RowOrCol::Both((1, 1)), MaxOrMin::MinAndMax::<u8, char>((8, 'H')), >, row, col, >}
impl_squares_iter! {UpperLeftSquare, RowOrCol::Both((-1, 1)), MaxOrMin::MinAndMax::<u8, char>((8, 'A')), >, row, col, <}
#[cfg(test)]
mod tests {
    use super::*;
    type Coords = ChessSquareCoordinates;
    /*
      static bottom_right: Coords = Coords::from_str("H1").unwrap();
      static top_left: Coords = Coords::from_str("A8").unwrap();
      static top_right: Coords = Coords::from_str("H8").unwrap();
       // just name left after coordinate
    */
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

