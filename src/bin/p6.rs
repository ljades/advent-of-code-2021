use std::collections::HashMap;

use advent_of_code_2021::inputs::read_input;

const LANTERNFISH_CYCLE_START: u8 = 6;
const LANTERNFISH_NEW_CYCLE_START: u8 = 8;

#[derive(Clone, Copy)]
struct FishGroup{
    size: usize,
    lifecycle: u8,
}

fn compress_fish_groups(fish_groups: &Vec<FishGroup>) -> Vec<FishGroup> {
    let mut lifecycle_to_size_hash: HashMap<u8, usize> = HashMap::new();
    for fish_group in fish_groups {
        let updated_size = fish_group.size + match lifecycle_to_size_hash.get(&fish_group.lifecycle) {
            Some(existing_size) => *existing_size,
            _ => 0,
        };
        lifecycle_to_size_hash.insert(fish_group.lifecycle, updated_size);
    }
    lifecycle_to_size_hash.iter()
        .map(|(lifecycle, size)| FishGroup{ lifecycle: *lifecycle, size: *size}).collect()
}

fn get_lanternfish_starting_counters(file_contents: String) -> Vec<u8> {
    file_contents.split(",").map(|counter_str| counter_str.parse::<u8>().unwrap()).collect()
}

fn cycle_lanternfish_one_day(lanternfish: &Vec<FishGroup>) -> Vec<FishGroup> {
    let mut updated_lanternfish: Vec<FishGroup> = Vec::new();
    let mut fresh_lanternfish: Vec<FishGroup> = Vec::new();
    for fish_group in lanternfish {
        if fish_group.lifecycle > 0 {
            updated_lanternfish.push(FishGroup{size: fish_group.size, lifecycle: fish_group.lifecycle - 1});
        } else {
            updated_lanternfish.push(FishGroup{size: fish_group.size, lifecycle: LANTERNFISH_CYCLE_START});
            fresh_lanternfish.push(FishGroup{size: fish_group.size, lifecycle: LANTERNFISH_NEW_CYCLE_START});
        }
    }
    updated_lanternfish.append(&mut fresh_lanternfish);
    updated_lanternfish
}

fn get_lanternfish_counters_after_days(lanternfish_starting_counters: &Vec<u8>, days: u32) -> Vec<FishGroup> {
    let mut lanternfish_groups: Vec<FishGroup> = lanternfish_starting_counters.iter().map(|counter| FishGroup{size: 1, lifecycle: *counter}).collect();
    for day in 0..days {
        lanternfish_groups = compress_fish_groups(&lanternfish_groups);
        println!("day {}, lanternfish size {}", day,
            lanternfish_groups.iter().fold(0, |fish_sum, lanternfish_group| fish_sum + lanternfish_group.size));
        lanternfish_groups = cycle_lanternfish_one_day(&lanternfish_groups);
    }
    lanternfish_groups
}

fn solve_part_1(lanternfish_starting_counters: &Vec<u8>) -> usize {
    let after_80_days = get_lanternfish_counters_after_days(lanternfish_starting_counters, 80);
    after_80_days.iter().fold(0, |fish_sum, lanternfish_group| fish_sum + lanternfish_group.size)
}

fn solve_part_2(lanternfish_starting_counters: &Vec<u8>) -> usize {
    let after_256_days = get_lanternfish_counters_after_days(lanternfish_starting_counters, 256);
    after_256_days.iter().fold(0, |fish_sum, lanternfish_group| fish_sum + lanternfish_group.size)
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i6.txt");
    let lanternfish_starting_counters = get_lanternfish_starting_counters(problem_raw_input);
    println!("{}", solve_part_1(&lanternfish_starting_counters));
    println!("{}", solve_part_2(&lanternfish_starting_counters));
}
