use std::fs;

fn read_input_file() -> Vec<Vec<i32>> {
    let input_file: String = fs::read_to_string("./input").expect("unable to read file");
    let mut temp: Vec<&str> = Vec::new();
    for line in input_file.lines() {
        temp.push(line);
    }
    //println!("temp: {:?}", temp);

    let mut log_array: Vec<Vec<i32>> = Vec::new();
    for lines in temp {
        let mut sub_array: Vec<i32> = Vec::new();
        for string_number in lines.split_whitespace().collect::<Vec<_>>() {
            let number = string_number.parse::<i32>().unwrap();
            sub_array.push(number);
        }
        log_array.push(sub_array);
    }
    //println!("string_log_array: {:?}", string_log_array);
    println!("{:?}", log_array);
    return log_array;
}

fn main() {
    let dataset: Vec<Vec<i32>> = read_input_file();
}
