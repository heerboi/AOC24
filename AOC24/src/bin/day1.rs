use std::io::Read;
use std::fs::File;

fn get_file_content(file_name: &str) -> String {
    let mut result = File::open(file_name).unwrap();

    let mut file_content = String::new();

    result.read_to_string(&mut file_content).unwrap();

    return file_content;
}

fn main() {

    // a1 - b1 + a2 - b2
    // a1 + a2 + a3 - (b1 + b2 + b3)
    
    let input = get_file_content("./input/day1.txt");


    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut sum = 0;

    for line in input.split("\r\n") {
        let nums: Vec<i32> = line.split("   ").map(|l| l.parse::<i32>().unwrap()).collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort();
    right.sort();

    for (i, left_num) in left.iter().enumerate() {
        sum += (left_num - right[i]).abs();
    }

    println!("The sum is {:?}", sum);
}