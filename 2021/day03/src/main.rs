use std::fs;

fn main() {
    let filename = "input";

    let content = fs::read_to_string(filename).expect("Cannot read file");

    let report: Vec<&str> = content
        .split_whitespace()
        .filter(|bits| !bits.is_empty())
        .collect();

    // Part 1
    // let mut gamma_rate = String::from("");
    // let mut epsilon_rate = String::from("");

    // for bit in 0..report[0].len() {
    //     let mut counter_1 = 0;
    //     let mut counter_0 = 0;
    //     for code in &report {
    //         let c = code.chars().nth(bit).expect("Cannot get char value");
    //         match c {
    //             '1' => { counter_1 += 1 },
    //             '0' => { counter_0 += 1 }
    //             _ => {}
    //         }
    //     }

    //     if counter_1 > counter_0 {
    //         gamma_rate.push('1');
    //         epsilon_rate.push('0');
    //     } else {
    //         gamma_rate.push('0');
    //         epsilon_rate.push('1');
    //     }

    // }

    // let gamma_value = isize::from_str_radix(&gamma_rate, 2).unwrap();
    // let epsilon_value = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    // println!("gamma_rate: {}", gamma_value);
    // println!("epsilon_rate: {}", epsilon_value);

    // println!("power_consumption: {}", gamma_value * epsilon_value);

    // Part 2
    let oxigen = find_oxigen_generator(report.clone());
    let oxigen_value = isize::from_str_radix(&oxigen, 2).unwrap();
    let co2 = co2_scrubber(report.clone());
    let co2_value = isize::from_str_radix(&co2, 2).unwrap();

    println!("val {}", oxigen_value * co2_value);
}

fn find_oxigen_generator(values: Vec<&str>) -> &str {
    let mut vec = values.clone();

    for i in 0..values[0].len() {
        let values_0: Vec<&str> = vec.clone()
            .into_iter()
            .filter(|val| val.chars().nth(i).expect("Cannot get char") == '0')
            .collect();

        let values_1: Vec<&str> = vec.clone()
            .into_iter()
            .filter(|val| val.chars().nth(i).expect("Cannot get char") == '1')
            .collect();

        if values_0.len() == values_1.len() {
            vec = values_1;
            continue;
        }

        if values_0.len() > values_1.len() {
            vec = values_0;
        } else {
            vec = values_1;
        }
    }

    vec[0]
}


fn co2_scrubber(values: Vec<&str>) -> &str{
    let mut vec = values.clone();

    for i in 0..values[0].len() {
        let values_0: Vec<&str> = vec.clone()
            .into_iter()
            .filter(|val| val.chars().nth(i).expect("Cannot get char") == '0')
            .collect();

        let values_1: Vec<&str> = vec.clone()
            .into_iter()
            .filter(|val| val.chars().nth(i).expect("Cannot get char") == '1')
            .collect();

        if vec.len() == 1 {
            return vec[0]
        }

        if values_0.len() == values_1.len() {
            vec = values_0;
            continue;
        }

        if values_0.len() < values_1.len() {
            vec = values_0;
        } else {
            vec = values_1;
        }
    }

    vec[0]
}