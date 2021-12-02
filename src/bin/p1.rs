use advent_of_code_2021::inputs::i1::DEPTHS_RAW;

fn get_depths_list_u32() -> Vec<u32> {
    let depths_list_str: Vec<&str> = DEPTHS_RAW.split("\n").collect();
    let mut depths_list_u32: Vec<u32> = Vec::new();
    for depth_str in depths_list_str {
        let depth_u32 = depth_str.parse::<u32>().unwrap();
        depths_list_u32.push(depth_u32);
    }

    depths_list_u32
}

#[allow(dead_code)]
fn get_total_increasing_depths() -> u32 {
    let depths_list_u32: Vec<u32> = get_depths_list_u32();

    let mut total_increasing = 0;
    let mut prev_depth: u32 = depths_list_u32[0];
    for next_depth in depths_list_u32 {
        if next_depth > prev_depth {
            total_increasing += 1;
        }
        prev_depth = next_depth;
    }
    total_increasing
}

const SLIDER_WINDOW_SIZE: usize = 3;

fn get_sliding_window_increasing_depths() -> u32 {
    let depths_list_u32: Vec<u32> = get_depths_list_u32();

    let mut next_window_end = SLIDER_WINDOW_SIZE;
    let mut total_increasing: u32 = 0;
    while next_window_end < depths_list_u32.len() {
        let prev_window_end = next_window_end - 1;
        let mut prev_window_sum = 0;
        for window_index in (0..SLIDER_WINDOW_SIZE).rev() {
            prev_window_sum += &depths_list_u32[prev_window_end - window_index];
        }

        let mut next_window_sum = 0;
        for window_index in (0..SLIDER_WINDOW_SIZE).rev() {
            next_window_sum += &depths_list_u32[next_window_end - window_index];
        }

        if next_window_sum > prev_window_sum {
            total_increasing += 1;
        }

        next_window_end += 1;
    }

    total_increasing
}

fn main(){
    // println!("{}", get_total_increasing_depths());
    println!("{}", get_sliding_window_increasing_depths());
}
