use std::{collections::HashMap, fs, iter::Enumerate};

fn main() {
    let input_file = fs::read_to_string("./input").expect("unable to read file");
    let input_individual: Vec<&str> = input_file.split_whitespace().collect();

    // initing the left and right list
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for (i, el) in input_individual.iter().enumerate() {
        let ri = i as i32;
        if ri % 2 == 0 {
            // even is left
            //println!("current number is even ({})", i);
            left.push(el.parse::<i32>().unwrap());
        } else {
            // odd is right
            //println!("current number is odd ({})", i);
            right.push(el.parse::<i32>().unwrap());
        }
        //println!("current element is {}", el);
    }

    left.sort();
    right.sort();

    //the hash mapening
    let mut left_map: HashMap<i32, usize> = HashMap::new();
    let mut right_map: HashMap<i32, usize> = HashMap::new();

    for (i, el) in left.iter().enumerate() {
        *left_map.entry(*el).or_default() += 1;
    }
    for (i, el) in right.iter().enumerate() {
        *right_map.entry(*el).or_default() += 1;
    }

    for (key, value) in &left_map {
        println!("{}: {}", key, value);
    }
    for (key, value) in &right_map {
        println!("{}: {}", key, value);
    }
    println!("{}", left_map.keys().len());
    println!("{}", right_map.keys().len());
}
