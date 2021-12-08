use std::cmp::min;

use advent_of_code_2021::inputs::read_input;

fn get_crabs_sorted(file_contents: String) -> Vec<i32> {
    let mut crabs: Vec<i32> = file_contents.split(",").map(|counter_str| counter_str.parse::<i32>().unwrap()).collect();
    crabs.sort_unstable();
    crabs
}

fn get_median(crabs_list: &Vec<i32>) -> i32 {
    crabs_list[crabs_list.len() / 2]
}

fn get_mean(crabs_list: &Vec<i32>) -> i32 {
    crabs_list.iter().fold(0, |sum, crab| sum + *crab) / crabs_list.len() as i32
}

fn get_fuel_cost(from: i32, to: i32, triangular: bool) -> i32 {
    if !triangular{
        (from - to).abs()
    } else {
        (from - to).abs() * ((from - to).abs() + 1) / 2
    }
}

fn sum_from_midpoint(crabs_list: &Vec<i32>, midpoint: i32, triangular: bool) -> i32 {
    crabs_list.iter().fold(0, |sum, crab| sum + get_fuel_cost(*crab, midpoint, triangular))
}

fn min_sum_from_midpoints(
    crabs_list: &Vec<i32>, triangular: bool, prev_midpoint: i32, prev_min_sum: i32, increment: i32
) -> i32 {
    let new_midpoint = prev_midpoint + increment;
    let sum_from_new_midpoint = sum_from_midpoint(crabs_list, new_midpoint, triangular);
    if sum_from_new_midpoint < prev_min_sum  {
        min(
            min_sum_from_midpoints(
                crabs_list, triangular, new_midpoint, sum_from_new_midpoint, increment
            ),
            prev_min_sum
        )
    } else {
        prev_min_sum
    }
}

fn solve_part_1(crabs_list: &Vec<i32>) -> i32 {
    let starting_point = get_median(crabs_list);
    let triangular = false;
    let starting_sum = sum_from_midpoint(crabs_list, starting_point, triangular);
    min(
        min_sum_from_midpoints(crabs_list, triangular, starting_point, starting_sum, -1),
        min_sum_from_midpoints(crabs_list, triangular, starting_point, starting_sum, 1)
    )
}

fn solve_part_2(crabs_list: &Vec<i32>) -> i32 {
    let starting_point = get_mean(crabs_list);
    let triangular = true;
    let starting_sum = sum_from_midpoint(crabs_list, starting_point, triangular);
    min(
        min_sum_from_midpoints(crabs_list, triangular, starting_point, starting_sum, -1),
        min_sum_from_midpoints(crabs_list, triangular, starting_point, starting_sum, 1)
    )
}


fn main() {
    let problem_raw_input = read_input("src/inputs/i7.txt");
    let crabs_sorted = get_crabs_sorted(problem_raw_input);
    println!("{}", solve_part_1(&crabs_sorted));
    println!("{}", solve_part_2(&crabs_sorted));
}
