use std::fs;

fn main() {
    let filename = "input";

    let content = fs::read_to_string(filename).expect("Cannot read file");

    let measurements: Vec<i32> = content
        .split('\n')
        .filter(|val| !val.is_empty())
        .map(|str| str.parse().expect("Cannot parse value"))
        .collect();

    let mut increased_times = 0;

    // Part 1
    // for i in 1..measurements.len() {
    //     if measurements[i] > measurements[i - 1] {
    //         increased_times += 1;
    //     }
    // }

    // println!("increased times --> {}", increased_times);

    // Part 2
    let mut prev_sum = 0;
    for i in 0..measurements.len() - 2 {
        let sum = measurements[i] + measurements[i + 1] + measurements[i + 2];

        if prev_sum == 0 {
            prev_sum = sum;
            continue;
        }


        if sum > prev_sum {
            increased_times += 1;
        }

        prev_sum = sum;
    }

    println!("increased times --> {}", increased_times);
}
