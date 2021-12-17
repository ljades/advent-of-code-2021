use std::collections::HashMap;

use advent_of_code_2021::inputs::read_input;

const NEST_TUPLES: [(char, char); 4] = [
    ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')
];

#[derive(Copy, Clone)]
struct SyntaxScores(u32, u64);

enum SyntaxError {
    Corrupted(char),
    Incomplete(String)
}

fn evaluate_line(bad_line: String, nest_map: &HashMap<char, char>) -> SyntaxError {
    let bad_line_chars = bad_line.chars();
    let mut char_stack: Vec<char> = Vec::new();
    let mut nest_level: u8 = 0;

    for next_char in bad_line_chars.into_iter() {
        if nest_map.contains_key(&next_char) {
            nest_level += 1;
            char_stack.push(next_char);
        } else {
            let last_nest_char = char_stack.pop().unwrap();
            nest_level -= 1;
            match nest_map.get(&last_nest_char) {
                Some(unnest_char) => {
                    if next_char != *unnest_char {
                        return SyntaxError::Corrupted(next_char);
                    }
                }
                None => panic!("We shouldn't have gotten here")
            }
        }
    }

    if nest_level > 0 {
        let mut completing_string = String::new();
        while let Some(curr_incomplete_char) = char_stack.pop() {
            completing_string.push(*nest_map.get(&curr_incomplete_char).unwrap())
        }
        return SyntaxError::Incomplete(completing_string);
    }
    panic!("This line is fine! What gives?")
}

fn get_corrupt_score(corrupt_char: char) -> u32 {
    match corrupt_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("bad char {}", corrupt_char)
    }
}

fn get_incomplete_score(completing_string: String) -> u64 {
    completing_string.chars().fold(0, |sum, completing_char| {
        (sum * 5) + match completing_char {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("bad char {}", completing_char)
        }
    })
}

fn evaluate_lines(bad_lines: &Vec<String>) -> SyntaxScores {
    let nest_map = HashMap::from(NEST_TUPLES);
    let mut incomplete_scores: Vec<u64> = Vec::new();
    let mut syntax_scores_final = bad_lines.iter().fold(
        SyntaxScores(0, 0), |syntax_scores, bad_line| {
            match evaluate_line(bad_line.clone(), &nest_map) {
                SyntaxError::Corrupted(bad_char) => {
                    SyntaxScores(syntax_scores.0 + get_corrupt_score(bad_char), syntax_scores.1)
                },
                SyntaxError::Incomplete(completing_string) => {
                    incomplete_scores.push(get_incomplete_score(completing_string));
                    syntax_scores
                }
            }
        }
    );
    incomplete_scores.sort_unstable();
    syntax_scores_final.1 = incomplete_scores[incomplete_scores.len() / 2];

    syntax_scores_final
}


fn get_bad_lines(file_contents: String) -> Vec<String> {
    file_contents.split("\n").map(|line_str| String::from(line_str)).collect()
}


fn main() {
    let problem_raw_input = read_input("src/inputs/i10.txt");
    let bad_lines = get_bad_lines(problem_raw_input);
    let syntax_scores_final = evaluate_lines(&bad_lines);
    println!("{}", syntax_scores_final.0);
    println!("{}", syntax_scores_final.1);
}
