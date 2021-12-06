use std::collections::HashMap;

use regex::Regex;

use advent_of_code_2021::inputs::read_input;


const BINGO_BOARD_SIZE: usize = 5;

#[derive(Copy, Clone)]
struct Position(usize, usize);

#[derive(Copy, Clone)]
struct BingoTile {
    value: u8,
    is_hit: bool,
}

impl BingoTile {
    fn new() -> BingoTile {
        BingoTile{ value: 0, is_hit: false }
    }
}

struct BingoBoard {
    grid: [[BingoTile; BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE],
    // Because this is advent of code I have a slight feeling I'm gonna need to optimize for p2,
    // and that means not checking up to 25 values every time we need to search for a hit.
    // This value_mapping will allow us the search the board with just one check
    // for position we want.
    // We still want to keep the grid above, however, as it will allow fast checking for wins.
    value_mapping: HashMap<u8, Position>,
    is_won: bool,
}

impl BingoBoard {
    fn hit(&mut self, value: u8) -> Option<Position> {
        match self.value_mapping.get(&value) {
            Some(position) => {
                self.grid[position.0][position.1].is_hit = true;
                return Some(position.clone());
            }
            _ => None,
        }
    }

    fn check_win(&self, hit_position: &Position) -> bool {
        // Check vertical
        let mut is_vertical_win = true;
        for vert_index in 0..BINGO_BOARD_SIZE {
            if !self.grid[vert_index][hit_position.1].is_hit {
                is_vertical_win = false;
                break;
            }
        }
        if is_vertical_win { return true; }
        // Check horizontal
        let mut is_horizontal_win = true;
        for horiz_index in 0..BINGO_BOARD_SIZE {
            if !self.grid[hit_position.0][horiz_index].is_hit {
                is_horizontal_win = false;
                break;
            }
        }
        if is_horizontal_win { return true; }
        false
    }

    fn mark_won(&mut self) {
        self.is_won = true;
    }

    fn reset(&mut self) {
        for vert_index in 0..BINGO_BOARD_SIZE {
            for horiz_index in 0..BINGO_BOARD_SIZE {
                self.grid[vert_index][horiz_index].is_hit = false;
            }
        }
    }

    fn get_board_score(&self, just_hit_value: u8) -> u32 {
        self.grid.iter()
            .fold(0, |unmarked_score: u32, row| {
                unmarked_score + row.iter().fold(0, |unmarked_score: u32, tile| {
                    match tile.is_hit {
                        true => unmarked_score,
                        false => unmarked_score + tile.value as u32,
                    }
                })
            })
            * just_hit_value as u32
    }
}

impl From<&str> for BingoBoard {
    fn from(bingo_str_block: &str) -> Self {
        let mut board = BingoBoard {
            grid: [[BingoTile::new(); BINGO_BOARD_SIZE]; BINGO_BOARD_SIZE],
            value_mapping: HashMap::new(),
            is_won: false,
        };

        let re = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
        let block_to_captures = re.captures_iter(bingo_str_block);
        for (row_index, capture) in block_to_captures.enumerate() {
            if row_index > BINGO_BOARD_SIZE { panic!("a parsed bingo block was greater than 5 lnes!"); }
            for col_index in 0..BINGO_BOARD_SIZE {
                let tile_value = capture[col_index + 1].parse::<u8>().unwrap();
                board.grid[row_index][col_index].value = tile_value;
                board.value_mapping.insert(tile_value, Position(row_index, col_index));
            }
        }
        board
    }
}


fn get_draws_and_boards_from_input(file_contents: String) -> (Vec<u8>, Vec<BingoBoard>) {
    let file_contents_components: Vec<&str> = file_contents.split("\n\n").collect();
    let draws_raw = &file_contents_components[0];
    let boards_raw = &file_contents_components[1..];

    let draws: Vec<u8> = draws_raw.split(",").map(|draw_str| draw_str.parse::<u8>().unwrap()).collect();
    let boards = boards_raw.iter()
        .map(|&board_str_block| BingoBoard::from(board_str_block)).collect();
    (draws, boards)
}

fn find_winning_board(draws: &Vec<u8>, boards: &mut Vec<BingoBoard>) -> Option<(usize, u8)> {
    for curr_draw in draws.iter() {
        for (curr_board_index, curr_board) in boards.iter_mut().enumerate() {
            let hit_position = curr_board.hit(*curr_draw);
            match hit_position {
                Some(position) => {
                    if curr_board.check_win(&position) {
                        return Some((curr_board_index, *curr_draw));
                    }
                }
                None => (),
            }
        }
    }
    None
}

fn find_final_board(draws: &Vec<u8>, boards: &mut Vec<BingoBoard>) -> Option<(usize, u8)> {
    //init step
    let mut last_won_index: Option<usize> = None;
    for curr_draw in draws.iter() {
        // filter step
        for (curr_board_index, curr_board) in boards.iter_mut().enumerate() {
            if !curr_board.is_won {
                let hit_position = curr_board.hit(*curr_draw);
                match hit_position {
                    Some(position) => {
                        if curr_board.check_win(&position) {
                            curr_board.mark_won();
                            last_won_index = Some(curr_board_index);
                        }
                    }
                    None => (),
                }
            }
        }
        if boards.iter().all(|board| board.is_won) {
            return Some((last_won_index.unwrap(), *curr_draw));
        }
    }
    None
}

fn solve_part_1(draws: &Vec<u8>, boards: &mut Vec<BingoBoard>) -> u32 {
    let winning_board_and_draw = find_winning_board(draws, boards);
    match winning_board_and_draw {
        None => { panic!("no board index found after all draws!"); }
        Some((board_index, draw)) => boards[board_index].get_board_score(draw),
    }
}

fn solve_part_2(draws: &Vec<u8>, boards: &mut Vec<BingoBoard>) -> u32 {
    let winning_board_and_draw = find_final_board(draws, boards);
    match winning_board_and_draw {
        None => { panic!("no board index found after all draws!"); }
        Some((board_index, draw)) => boards[board_index].get_board_score(draw),
    }
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i4.txt");
    let (draws, mut boards) = get_draws_and_boards_from_input(problem_raw_input);
    println!("{}", solve_part_1(&draws, &mut boards));
    for board in boards.iter_mut() {
        board.reset();
    }
    println!("{}", solve_part_2(&draws, &mut boards));
}
