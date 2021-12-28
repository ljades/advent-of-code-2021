use std::collections::HashSet;

use advent_of_code_2021::inputs::read_input;
use advent_of_code_2021::shared::positions::{ Position, DIRECTIONS };

const FLASH_THRESHOLD: u8 = 9;

struct Dumbos {
    grid: Vec<Vec<u8>>,
    num_flashes: u32,
}

impl Clone for Dumbos {
    fn clone(&self) -> Dumbos {
        let mut grid_clone = Vec::new();
        for grid_row in self.grid.iter() {
            grid_clone.push(grid_row.clone());
        }
        Dumbos { grid: grid_clone, num_flashes: 0 }
    }
}

impl From<Vec<String>> for Dumbos {
    fn from(digit_string_grid: Vec<String>) -> Self {
        Dumbos {
            grid: digit_string_grid.iter().map(
                |digit_line| {
                    digit_line.chars().into_iter().map(
                        |digit_char| digit_char.to_digit(10).unwrap() as u8
                    ).collect()
                }
            ).collect(),
            num_flashes: 0,
        }
    }
}

impl Dumbos {
    fn debug_print(&self) {
        for row in self.grid.iter() {
            println!("{:?}", row);
        }
        println!("num flashes so far: {}", self.num_flashes);
    }

    fn flash_dfs(&mut self, initial_flashes: &Vec<Position>) {
        let row_size = self.grid.len();
        let col_size = self.grid[0].len();
        let mut flash_stack: Vec<Position> = initial_flashes.clone();
        let mut flashed: HashSet<Position> = HashSet::from_iter(initial_flashes.clone().into_iter());
        while let Some(curr_flashed_position) = flash_stack.pop() {
            let neighbors: Vec<Option<Position>> = DIRECTIONS.iter().map(
                |direction| curr_flashed_position.neighbor(direction, row_size, col_size)
            ).collect();
            for neighbor in neighbors.iter() {
                match neighbor {
                    Some(neighbor_pos) => {
                        self.grid[neighbor_pos.0][neighbor_pos.1] += 1;
                        if !flashed.contains(&neighbor_pos) && self.grid[neighbor_pos.0][neighbor_pos.1] > FLASH_THRESHOLD {
                            flashed.insert(*neighbor_pos);
                            flash_stack.push(*neighbor_pos);
                        }
                    },
                    None => (),
                }
            }
        }
    }

    fn reset_flashes_and_is_sync(&mut self) -> bool {
        // Reset the flashes found, add them to the total number, and return true if all flashed
        let row_size = self.grid.len();
        let col_size = self.grid[0].len();
        let mut all_flashed = true;

        for row_index in 0..row_size {
            for col_index in 0..col_size {
                if self.grid[row_index][col_index] > FLASH_THRESHOLD {
                    self.grid[row_index][col_index] = 0;
                    self.num_flashes += 1;
                } else {
                    all_flashed = false;
                }
            }
        }
        all_flashed
    }

    fn step_and_is_sync(&mut self) -> bool {
        let row_size = self.grid.len();
        let col_size = self.grid[0].len();
        let mut initial_flashes: Vec<Position> = Vec::new();
        for row_index in 0..row_size {
            for col_index in 0..col_size {
                self.grid[row_index][col_index] += 1;
                if self.grid[row_index][col_index] > FLASH_THRESHOLD {
                    initial_flashes.push(Position(row_index, col_index));
                }
            }
        }
        self.flash_dfs(&initial_flashes);
        self.reset_flashes_and_is_sync()
    }
}

fn get_dumbos(file_contents: String) -> Dumbos {
    let file_as_string_vec: Vec<String> = file_contents.split("\n").map(|line_str| String::from(line_str)).collect();
    Dumbos::from(file_as_string_vec)
}

fn solve_part_1(dumbos: &Dumbos) -> u32 {
    let mut dumbos_clone = dumbos.clone();
    dumbos_clone.debug_print();
    for _ in 0..100 {
        dumbos_clone.step_and_is_sync();
        dumbos_clone.debug_print();
    }
    dumbos_clone.num_flashes
}

fn solve_part_2(dumbos: &Dumbos) -> u32 {
    let mut dumbos_clone = dumbos.clone();
    let mut step_counter: u32 = 1;
    while !dumbos_clone.step_and_is_sync() {
        println!("Step: {} done. Not in sync", step_counter);
        step_counter += 1;
    }
    step_counter
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i11.txt");
    let dumbos = get_dumbos(problem_raw_input);
    println!("{}", solve_part_1(&dumbos));
    println!("{}", solve_part_2(&dumbos));
}
