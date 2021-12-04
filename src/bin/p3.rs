use advent_of_code_2021::inputs::read_input;

// TODO: Imrovements:
//       - It's a constant time reduction, but no need to find all, including irrelevant, most and least common
//          bits on each run for Part 2
//       - Arguably convert the binary strings to bytes at the beginning, do all future comparisons
//         on that. Then, at the final stage, convert to String, and then from binary to u32

struct MostAndLeastCommonBitStrings(String, String);

fn read_binary_strings_from_input(file_contents: String) -> Vec<String> {
    let split_contents: Vec<&str> = file_contents.split("\n").collect();
    split_contents.into_iter().map(|raw_str| String::from(raw_str)).collect()
}

fn get_most_and_least_common_bit_strings(binaries: &Vec<&String>) -> MostAndLeastCommonBitStrings {
    // There are multiple layered steps to this. Here's the process overall:
    // 0. To keep track of most common binary digit, I'm using a vector of counters that can
    //    swing positive or negative. As a 0 digit is logged, -1 gets added, and as a 1 digit
    //    is logged, +1 gets added. If the final value is positive, then 1 was the most common
    //    digit. Otherwise, 0 was more common.
    // 1. Each line of input is a binary string. We want to convert this to a vector of modifiers
    //    to the counters. So we map it as such.
    // 2. In order to merge the modifiers into the current counters, we zip them together, then
    //    map the new tuple by summing the two elements.
    // 3. We do this process over a fold on all of the binary strings to log the full digit counters
    let digit_counters = binaries.into_iter().fold(
        vec![0; binaries[0].len()],
        |current_counters, &binary_line| {
            let digit_modifiers: Vec<i32> = binary_line.chars().map(|current_character| {
                match current_character {
                    '0' => -1,
                    '1' => 1,
                    _ => panic!("non-binary char found"),
                }
            }).collect();
            current_counters.into_iter()
                .zip(digit_modifiers).map(|to_merge_tuple| to_merge_tuple.0 + to_merge_tuple.1).collect()
        });

    let mut most_common: String = String::new();
    let mut least_common: String = String::new();
    for digit_counter in digit_counters.iter() {
        if *digit_counter > 0 {
            most_common.push('1');
            // Edge case! if the digit counter equals the length of the whole list of binaries,
            // then there are ZERO of the other digit, which means the least common is equal to
            // the most common!
            if *digit_counter as usize == binaries.len() { least_common.push('1') } else { least_common.push('0'); }
        } else if *digit_counter < 0 {
            most_common.push('0');
            if (*digit_counter * -1) as usize == binaries.len() { least_common.push('0') } else { least_common.push('1'); }
        } else {
            most_common.push('1');
            least_common.push('0');
        }
    }
    MostAndLeastCommonBitStrings(most_common, least_common)
}

fn solve_part_1(binaries: &Vec<&String>) -> u32 {
    let most_and_least_common = get_most_and_least_common_bit_strings(binaries);
    let (gamma_rate, epsilon_rate) = (
        String::from(&most_and_least_common.0),
        String::from(&most_and_least_common.1));
    u32::from_str_radix(&gamma_rate, 2).unwrap() * u32::from_str_radix(&epsilon_rate, 2).unwrap()
}

fn solve_part_2(binaries: &Vec<&String>) -> u32 {
    let mut filter_index: usize = 0;
    let mut filtered_binaries: Vec<&String> = binaries.to_vec();
    // Calculate O2 rating
    while filter_index < binaries[0].len() && filtered_binaries.len() > 1 {
        let most_and_least_common = get_most_and_least_common_bit_strings(&filtered_binaries);
        // Most common bit in a given position, in byte form for easy comparison
        let filter_bit_as_byte = most_and_least_common.0.as_bytes()[filter_index];

        filtered_binaries = filtered_binaries.into_iter()
            .filter(|&filtered_binary| filtered_binary.as_bytes()[filter_index] == filter_bit_as_byte)
            .collect();
        filter_index += 1;
    }
    if filtered_binaries.len() != 1 {
        panic!("didn't filter properly! remaining binary lines number is {}", filtered_binaries.len());
    }
    let o2_generator_rating = u32::from_str_radix(filtered_binaries[0], 2).unwrap();

    // Calculate C2 Scrubber Rating
    filtered_binaries = binaries.to_vec();
    filter_index = 0;
    while filter_index < binaries[0].len() && filtered_binaries.len() > 1 {
        let most_and_least_common = get_most_and_least_common_bit_strings(&filtered_binaries);
        // Least common bit in a given position, in byte form for easy comparison
        let filter_bit_as_byte = most_and_least_common.1.as_bytes()[filter_index];

        filtered_binaries = filtered_binaries.into_iter()
            .filter(|&filtered_binary| filtered_binary.as_bytes()[filter_index] == filter_bit_as_byte)
            .collect();
        filter_index += 1;
    }
    if filtered_binaries.len() != 1 {
        panic!("didn't filter properly! remaining binary lines number is {}", filtered_binaries.len());
    }
    let co2_scrubber_rating = u32::from_str_radix(filtered_binaries[0], 2).unwrap();

    o2_generator_rating * co2_scrubber_rating
}

fn main() {
    let problem_raw_input = read_input("src/inputs/i3.txt");
    let interpreted_binaries = read_binary_strings_from_input(problem_raw_input);
    let interpreted_binaries_refs = interpreted_binaries.iter().collect();
    println!("{}", solve_part_1(&interpreted_binaries_refs));
    println!("{}", solve_part_2(&interpreted_binaries_refs));
}
