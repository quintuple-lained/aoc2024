use std::{fs, slice::EscapeAscii};

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
    // if left.len() == right.len() {
    //     // println!("it worked!");
    //     // println!("left length = {}", left.len());
    //     // println!("right length = {}", right.len());
    // } else {
    //     // println!("failure :pensive:");
    //     // println!("left length = {}", left.len());
    //     // println!("right length = {}", right.len());
    // }

    left.sort();
    right.sort();

    let mut distances: Vec<i32> = Vec::new();
    let mut dist: i32 = 0;
    for (i, el) in left.iter().enumerate() {
        if el > &right[i] {
            dist = el - right[i];
        } else if &right[i] > el {
            dist = right[i] - el;
        } else {
            dist = 0;
            //println!("is this really 0?: {} - {} = {}", el, right[i], dist);
        }
        distances.push(dist);
    }

    //    println!("{:?}", distances);

    let mut total_distance: i32 = 0;
    for (i, el) in distances.iter().enumerate() {
        //      println!("{}", i);
        let new_total = total_distance + el;
        total_distance = new_total
    }
    println!("{}", total_distance)
}
