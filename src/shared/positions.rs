#[derive(Copy, Clone, Hash)]
pub struct Position(pub usize, pub usize);

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Position {}

pub struct Direction(isize, isize);

impl Position {
    pub fn neighbor(
        &self, direction: &Direction, y_bound: usize, x_bound: usize
    ) -> Option<Position> {
        let new_y: isize = self.0 as isize + direction.0;
        let new_x: isize = self.1 as isize + direction.1;
        if new_y >= 0 && new_y < y_bound as isize && new_x >= 0 && new_x < x_bound as isize {
            Some(Position(new_y as usize, new_x as usize))
        } else {
            None
        }
    }
}

pub const NON_DIAG_DIRECTIONS: [Direction; 4] = [Direction(-1, 0), Direction(1, 0), Direction(0, -1), Direction(0, 1)];
pub const DIRECTIONS: [Direction; 8] = [
    Direction(-1, 0), Direction(1, 0), Direction(0, -1), Direction(0, 1),
    Direction(-1, -1), Direction(1, -1), Direction(-1, 1), Direction(1, 1),
];
