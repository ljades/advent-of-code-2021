use std::collections::HashSet;

use advent_of_code_2021::inputs::read_input;


#[derive(Copy, Clone, Hash)]
struct Position(usize, usize);

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Position {}

struct Direction(isize, isize);

impl Position {
    fn neighbor(
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

const DIRECTIONS: [Direction; 4] = [Direction(-1, 0), Direction(1, 0), Direction(0, -1), Direction(0, 1)];

struct HeightMap {
    grid: Vec<Vec<u8>>,
    low_points: Vec<Position>
}

impl HeightMap {
    fn new(grid: Vec<Vec<u8>>) -> HeightMap {
        let mut low_points = Vec::new();
        let row_size = grid.len();
        let col_size = grid[0].len();
        for row_index in 0..row_size {
            for col_index in 0..col_size {
                let position = Position(row_index, col_index);
                let neighbors: Vec<Option<Position>> = DIRECTIONS.iter().map(
                    |direction| position.neighbor(direction, row_size, col_size)
                ).collect();
                if !neighbors.iter().any(|neighbor| {
                        match neighbor {
                            Some(neighbor_pos) => {
                                grid[neighbor_pos.0][neighbor_pos.1] <= grid[row_index][col_index]
                            },
                            None => false,
                        }
                    }) {
                    low_points.push(position);
                }
            }
        }

        HeightMap{ grid, low_points }
    }

    fn get_basin_size(&self, low_point: &Position) -> u32 {
        let mut marked_positions: HashSet<Position> = HashSet::new();
        let low_point_copy = *low_point;
        let row_size = self.grid.len();
        let col_size = self.grid[0].len();

        let mut dfs_stack = vec![low_point_copy];
        marked_positions.insert(low_point_copy);
        let mut basin_size: u32 = 0;

        while let Some(curr_position) = dfs_stack.pop() {
            basin_size += 1;
            marked_positions.insert(curr_position);
            let neighbors: Vec<Option<Position>> = DIRECTIONS.iter().map(
                |direction| curr_position.neighbor(direction, row_size, col_size)
            ).collect();
            for neighbor in neighbors.iter() {
                match neighbor {
                    Some(neighbor_pos) => {
                        if !marked_positions.contains(&neighbor_pos) && self.grid[neighbor_pos.0][neighbor_pos.1] < 9 {
                            marked_positions.insert(*neighbor_pos);
                            dfs_stack.push(*neighbor_pos);
                        }
                    },
                    None => (),
                }
            }
        }

        basin_size
    }
}

fn get_height_map(file_contents: String) -> HeightMap {
    let grid: Vec<Vec<u8>> = file_contents.split("\n").map(
        |file_line| {
            let grid_line: Vec<u8> = String::from(file_line).chars().map(
                |character| character.to_digit(10).unwrap() as u8
            ).collect();
            grid_line
        }
    ).collect();
    HeightMap::new(grid)
}

fn solve_part_1(height_map: &HeightMap) -> u32 {
    height_map.low_points.iter().fold(0, |risk_level_sum, low_point| {
        risk_level_sum + height_map.grid[low_point.0][low_point.1] as u32 + 1
    } )
}

fn solve_part_2(height_map: &HeightMap) -> u32 {
    let mut basin_sizes: Vec<u32> = height_map.low_points.iter().map(
        |low_point| height_map.get_basin_size(low_point)
    ).collect();
    println!("{:?}", basin_sizes);
    basin_sizes.sort_unstable_by(|first, second| second.cmp(first));
    println!("{:?}", basin_sizes);
    basin_sizes[0..3].iter().fold(1, |basin_product, basin_size| basin_product * basin_size)
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i9.txt");
    let height_map = get_height_map(problem_raw_input);
    println!("{}", solve_part_1(&height_map));
    println!("{}", solve_part_2(&height_map));
}
