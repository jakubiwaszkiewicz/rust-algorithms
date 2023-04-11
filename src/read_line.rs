use std::io::{self, BufRead};

pub (crate) fn _string() -> String {
    println!("Input string.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_owned()
}

pub (crate) fn _integer() -> i32 {
    println!("Input integer.");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read input");
    let input: i32 = input_string.trim().parse().expect("Invalid input. Please input integer number.");
    input
}

pub (crate) fn _vec_sorted_ascending() -> Vec<usize> {
    println!("Input vector");
    let input_string = match io::stdin().lock().lines().next() {
        Some(Ok(input)) => input,
        _ => panic!("Error reading input."),
    };
    let mut input_vec: Vec<usize> = input_string
        .split_whitespace()
        .map(|x| match x.parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid input. Please input numbers that are sorted in ascending order with spaces between them."),
        })
        .collect();
    input_vec.sort();
    return input_vec;
}

pub (crate) fn _vec_unsorted() -> Vec<usize> {
    println!("Input vector");
    let input_string = match io::stdin().lock().lines().next() {
        Some(Ok(input)) => input,
        _ => panic!("Error reading input."),
    };
    let input_vec: Vec<usize> = input_string
        .split_whitespace()
        .map(|x| match x.parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid input. Please input numbers with spaces between them."),
        })
        .collect();
    return input_vec;
}