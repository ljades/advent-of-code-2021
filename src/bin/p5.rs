use std::{collections::HashMap, hash::Hash};

use regex::Regex;

use advent_of_code_2021::inputs::read_input;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i32, i32);

impl From<&str> for Position {
    fn from(position_str: &str) -> Self {
        let position_regex: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
        let cap = position_regex.captures(position_str);
        match cap {
            Some(successful_cap) => {
                Position(successful_cap[1].parse::<i32>().unwrap(),
                    successful_cap[2].parse::<i32>().unwrap())
            }
            _ => panic!("position capture on {} didn't work properly", position_str),
        }
    }
}

struct LineSegment {
    start_point: Position,
    end_point: Position,
}

impl LineSegment {
    fn get_x_increment(&self) -> i32 {
        if self.start_point.0 < self.end_point.0 {
            1
        } else if self.start_point.0 > self.end_point.0 {
            -1
        }
        else {
            0
        }
    }

    fn get_y_increment(&self) -> i32 {
        if self.start_point.1 < self.end_point.1 {
            1
        } else if self.start_point.1 > self.end_point.1 {
            -1
        }
        else {
            0
        }
    }

    fn is_diagonal(&self) -> bool {
        self.start_point.0 != self.end_point.0 && self.start_point.1 != self.end_point.1
    }
}

impl From<&str> for LineSegment {
    fn from(line_segment_str: &str) -> Self {
        let line_segment_str_split: Vec<&str> = line_segment_str.split(" -> ").collect();
        LineSegment {
            start_point: Position::from(line_segment_str_split[0]),
            end_point: Position::from(line_segment_str_split[1])
        }
    }
}

struct PointsHash {
    marked_points: HashMap<Position, u32>,
    use_diagonals: bool,
}

impl PointsHash {
    fn mark_line(&mut self, line_segment: &LineSegment) {
        let mut curr_pos = Position(line_segment.start_point.0, line_segment.start_point.1);
        let increments = (line_segment.get_x_increment(), line_segment.get_y_increment());

        while curr_pos != Position(line_segment.end_point.0 + increments.0, line_segment.end_point.1 + increments.1) {
            let overlap_val = 1 + match self.marked_points.get(&curr_pos) {
                Some(current_position_overlap) => {
                    *current_position_overlap
                }
                _ => {
                    0
                }
            };
            self.marked_points.insert(curr_pos, overlap_val);

            curr_pos = Position(curr_pos.0 + increments.0, curr_pos.1 + increments.1)
        }
    }

    fn mark_line_set(&mut self, line_segments: &Vec<LineSegment>) {
        for line_segment in line_segments.iter() {
            if !line_segment.is_diagonal() || self.use_diagonals {
                self.mark_line(line_segment);
            }
        }
    }

    fn get_num_points_overlap(&self, threshold: u32) -> u32 {
        self.marked_points.values().fold(0, |overlap_above_threshold, value| {
            overlap_above_threshold + if *value >= threshold {
                1
            } else {
                0
            }
        })
    }
}

fn get_line_segments(file_contents: String) -> Vec<LineSegment> {
    file_contents.split("\n").map(|line_segment_str| LineSegment::from(line_segment_str)).collect()
}

fn solve_both_parts(line_segments: &Vec<LineSegment>, use_diagonals: bool) -> u32 {
    let mut points_hash = PointsHash {
        marked_points: HashMap::new(),
        use_diagonals,
    };
    points_hash.mark_line_set(line_segments);
    points_hash.get_num_points_overlap(2)
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i5.txt");
    let line_segments = get_line_segments(problem_raw_input);
    println!("{}", solve_both_parts(&line_segments, false));
    println!("{}", solve_both_parts(&line_segments, true));
}
