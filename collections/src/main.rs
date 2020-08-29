fn main() {
    let mut v = vec![1,2,3,4,5,6,7,8,9,10,10,9];
    let mean = mean(&v);
    println!("{}", mean);
    let median = median(&mut v);
    println!("{}", median);
    let mode = mode(&v);
    println!("{:#?}", mode);
}

fn mean(integers: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut total_integers = 0;
    for i in integers.iter() {
        sum += i;
        total_integers += 1;
    }
    (sum / total_integers).into()
}

fn median(integers: &mut Vec<i32>) -> i32 {
    integers.sort();
    let len = integers.len();
    let even = len % 2 == 0;
    if even {
        integers[len / 2]
    } else {
        (integers[len / 2] + integers[(len / 2) - 1]) / 2
    }
}

fn mode(integers: &Vec<i32>) -> Vec<i32> {
    let mut modes = vec![];
    let mut counts = HashMap::new();
    let mut highest_count = 0;
    for i in integers.iter() {
        let count = counts.entry(i).or_insert(1);
        *count += 1;
        if *count == highest_count {
            modes.push(*i);
        } else if *count > highest_count {
            highest_count = *count;
            modes = vec![*i];
        }
    }

    modes
}

use std::collections::HashMap;