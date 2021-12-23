use std::fs;

fn main() {
    let filename = "input";

    let content = fs::read_to_string(filename).expect("Cannot read file");

    let report: Vec<&str> = content
        .split_whitespace()
        .filter(|bits| !bits.is_empty())
        .collect();

    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");

    for bit in 0..report[0].len() {
        let mut counter_1 = 0;
        let mut counter_0 = 0;
        for code in &report {
            let c = code.chars().nth(bit).expect("Cannot get char value");
            match c {
                '1' => { counter_1 += 1 },
                '0' => { counter_0 += 1 }
                _ => {}
            }
        }

        if counter_1 > counter_0 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }

    }

    let gamma_value = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_value = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("gamma_rate: {}", gamma_value);
    println!("epsilon_rate: {}", epsilon_value);

    println!("power_consumption: {}", gamma_value * epsilon_value);
}
