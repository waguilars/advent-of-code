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

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increased_times += 1;
        }
    }

    println!("increased times --> {}", increased_times);
}
