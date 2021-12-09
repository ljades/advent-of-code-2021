use std::collections::{HashMap, HashSet};
use std::str;

use advent_of_code_2021::inputs::read_input;

struct DisplayDigit {
    bytes: Vec<u8>,
}

impl Clone for DisplayDigit {
    fn clone(&self) -> Self {
        DisplayDigit { bytes: self.bytes.clone() }
    }
}

impl From<&str> for DisplayDigit {
    fn from(digit_str: &str) -> Self {
        let mut digit_as_bytes = digit_str.as_bytes().to_vec();
        digit_as_bytes.sort_unstable();
        DisplayDigit{ bytes: digit_as_bytes }
    }
}

struct DisplayInfo{
    digit_set: Vec<DisplayDigit>,
    output: Vec<DisplayDigit>,
}

impl From<&str> for DisplayInfo{
    fn from(display_line: &str) -> Self {
        let display_line_split: Vec<&str> = display_line.split(" | ").collect();
        DisplayInfo {
            digit_set: display_line_split[0].split(" ").map(|digit_str| DisplayDigit::from(digit_str)).collect(),
            output: display_line_split[1].split(" ").map(|digit_str| DisplayDigit::from(digit_str)).collect(),
        }
    }
}

// Logic notes:
// - 0 has SIX segments
// - 1 is TWO segments. Always know 1.
// - 2 is FIVE segments. union of 1 and 2 - set of 2 => F
// - 3 is FIVE segments
// - 4 is FOUR segments
// - 5 is FIVE segments
// - 6 is SIX segments. Set of 8 - set of 6 => C
// - 7 is THREE segments. Set of 7's segments - set of 1's segments => A
// - 8 is SEVEN segments. set of 8 - set of 0 => D
// - 9 is SIX segments. set of 8 - set of 9 => E
//        set of 9 - set of 3 => B
// Final letter not solved for => G

// Step by step:
// Find 1, 4, 7, 8 because of unique segments.
// NOT NEEDED: Find A with 7 set - 1 set.
// Find C and D and E with the three six-segment pieces.
//      Let CDE = Union - Intersect of the three. Intersect of CDE and 1 is C.
//      1 - C is F. CDE - C is DE. DE - 4 is E. DE - E is D.
// Let 6 be the six segment piece without C.
// Let 9 be the six segment piece without E.
// Let 0 be the six segment piece without D.
// Let 2 be the five segment piece without F. Let 3 be ... without E.
//      Let 5 be ... without C AND without E.
// (Progress: We know 10 digits, five letters)
fn get_digit_map(digit_set: &Vec<DisplayDigit>) -> HashMap<Vec<u8>, u8> {
    // Here we implement the logic above
    // TODO: figure out hash references
    let one = digit_set.iter().find(|unknown_digit| unknown_digit.bytes.len() == 2).unwrap();
    let four = digit_set.iter().find(|unknown_digit| unknown_digit.bytes.len() == 4).unwrap();
    let seven = digit_set.iter().find(|unknown_digit| unknown_digit.bytes.len() == 3).unwrap();
    let eight = digit_set.iter().find(|unknown_digit| unknown_digit.bytes.len() == 7).unwrap();

    let unknown_six_segments: Vec<DisplayDigit> = digit_set.clone().into_iter()
        .filter(|unknown_digit| unknown_digit.bytes.len() == 6).collect();
    //let unknown_six_segments: Vec<DisplayDigit> =
    //    digit_set.iter().filter(|unknown_digit| unknown_digit.bytes.len() == 6)
    //    .map(|display_digit_ref| display_digit_ref.clone() ).collect();
    if unknown_six_segments.len() != 3 { panic!("number of six segment digits is wrong!")}

    let one_set: HashSet<u8> = one.bytes.clone().into_iter().collect();
    let four_set: HashSet<u8> = four.bytes.clone().into_iter().collect();

    let unknown_six_segments_sets: Vec<HashSet<u8>> =
        unknown_six_segments.clone().iter().map(|digit| digit.bytes.clone().into_iter().collect()).collect();
    let six_segments_union: HashSet<u8> = &(&unknown_six_segments_sets[0] | &unknown_six_segments_sets[1]) | &unknown_six_segments_sets[2];
    let six_segments_intersection: HashSet<u8> = &(&unknown_six_segments_sets[0] & &unknown_six_segments_sets[1]) & &unknown_six_segments_sets[2];
    let c_d_e_set = &six_segments_union - &six_segments_intersection;
    let c_set = &c_d_e_set & &one_set;
    let d_e_set = &c_d_e_set - &c_set;
    let e_set = &d_e_set - &four_set;
    let d_set = &d_e_set - &e_set;
    let c = c_set.into_iter().next().unwrap();
    let d = d_set.into_iter().next().unwrap();
    let e = e_set.into_iter().next().unwrap();
    let six = unknown_six_segments.iter().find(|digit| !digit.bytes.contains(&c)).unwrap().clone();
    let nine = unknown_six_segments.iter().find(|digit| !digit.bytes.contains(&e)).unwrap();
    let zero = unknown_six_segments.iter().find(|digit| !digit.bytes.contains(&d)).unwrap();

    let unknown_five_segments: Vec<&DisplayDigit> =
        digit_set.iter().filter(|unknown_digit| unknown_digit.bytes.len() == 5).collect();
    if unknown_five_segments.len() != 3 { panic!("number of six segment digits is wrong!")}

    let two = unknown_five_segments.iter().find(|digit| digit.bytes.contains(&e)).unwrap();
    let three = unknown_five_segments.iter().find(|digit| digit.bytes.contains(&c) && !digit.bytes.contains(&e)).unwrap();
    let five = unknown_five_segments.iter().find(|digit| !digit.bytes.contains(&c) && !digit.bytes.contains(&e)).unwrap();

    let mut digit_map: HashMap<Vec<u8>, u8> = HashMap::new();
    digit_map.insert(zero.bytes.clone(), 0);
    digit_map.insert(one.bytes.clone(), 1);
    digit_map.insert(two.bytes.clone(), 2);
    digit_map.insert(three.bytes.clone(), 3);
    digit_map.insert(four.bytes.clone(), 4);
    digit_map.insert(five.bytes.clone(), 5);
    digit_map.insert(six.bytes.clone(), 6);
    digit_map.insert(seven.bytes.clone(), 7);
    digit_map.insert(eight.bytes.clone(), 8);
    digit_map.insert(nine.bytes.clone(), 9);
    digit_map
}

fn get_display_infos(file_contents: String) -> Vec<DisplayInfo> {
    file_contents.split("\n").map(|display_info_line| DisplayInfo::from(display_info_line)).collect()
}

fn solve_part_1(display_infos: &Vec<DisplayInfo>) -> u32 {
    display_infos.iter().fold(0, |sum_of_easy, display_info| {
        let digit_map = get_digit_map(&display_info.digit_set);
        sum_of_easy + display_info.output.iter().fold(0, |single_line_sum, display_digit| {
            single_line_sum + match digit_map.get(&display_digit.bytes) {
                Some(&1) | Some(&4) | Some(&7) | Some(&8) => 1,
                Some(_) => 0,
                None => panic!("digit {} not found", str::from_utf8(&display_digit.bytes).unwrap())
            }
        })
    })
}

fn solve_part_2(display_infos: &Vec<DisplayInfo>) -> u32 {
    display_infos.iter().fold(0, |sum_of_easy, display_info| {
        let digit_map = get_digit_map(&display_info.digit_set);
        sum_of_easy + display_info.output.iter().fold(0, |single_line_sum, display_digit| {
            (single_line_sum * 10) + match digit_map.get(&display_digit.bytes) {
                Some(digit) => *digit as u32,
                None => panic!("digit {} not found", str::from_utf8(&display_digit.bytes).unwrap())
            }
        })
    })
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i8.txt");
    let display_infos = get_display_infos(problem_raw_input);
    println!("{}", solve_part_1(&display_infos));
    println!("{}", solve_part_2(&display_infos));
}
