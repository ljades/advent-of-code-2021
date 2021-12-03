use advent_of_code_2021::inputs::read_input;

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn instruction_tuple_to_instruction(instruction_tuple: (&str, &str)) -> Instruction {
    let val: u32 = instruction_tuple.1.parse::<u32>().unwrap();
    match instruction_tuple {
        ("forward", ..) => Instruction::Forward(val),
        ("down", ..) => Instruction::Down(val),
        ("up", ..) => Instruction::Up(val),
        (bad_instr, ..) => panic!("bad instruction {}", bad_instr)
    }
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn get_new_position(self, horizontal: i32, depth: i32, use_aim: bool) -> Position {
        match use_aim {
            false => Position{ horizontal: self.horizontal + horizontal, depth: self.depth + depth, aim: self.aim },
            true => {
                Position{ horizontal: self.horizontal + horizontal, depth: self.depth + (horizontal * self.aim), aim: self.aim + depth }
            },
        }
    }

    fn process_instruction(self, instruction: &Instruction, use_aim: bool) -> Position {
        match *instruction {
            Instruction::Forward(horizontal) => self.get_new_position(horizontal as i32, 0, use_aim),
            Instruction::Down(depth) => self.get_new_position(0, depth as i32, use_aim),
            Instruction::Up(negative_depth) => self.get_new_position(0, -1 * negative_depth as i32, use_aim),
        }
    }
}

fn read_instructions_from_input(file_contents: String) -> Vec<Instruction> {
    let instruction_raw_lines: Vec<&str> = file_contents.split("\n").collect();

    instruction_raw_lines.into_iter()
        .map(| raw_line | raw_line.split(" "))
        .map(| mut split_line | (split_line.next().unwrap(), split_line.next().unwrap()))
        .map(instruction_tuple_to_instruction).collect()
}

fn solve_part_1(instructions: &Vec<Instruction>) -> i32 {
    let final_position = instructions.into_iter()
        .fold(Position{ horizontal: 0, depth: 0, aim: 0 },
            | pos, instruction | pos.process_instruction(instruction, false));
    final_position.horizontal * final_position.depth
}

fn solve_part_2(instructions: &Vec<Instruction>) -> i32 {
    let final_position = instructions.into_iter()
        .fold(Position{ horizontal: 0, depth: 0, aim: 0 },
            | pos, instruction | pos.process_instruction(instruction, true));
    final_position.horizontal * final_position.depth
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i2.txt");
    let interpreted_instructions = read_instructions_from_input(problem_raw_input);
    println!("{}", solve_part_1(&interpreted_instructions));
    println!("{}", solve_part_2(&interpreted_instructions));
}
