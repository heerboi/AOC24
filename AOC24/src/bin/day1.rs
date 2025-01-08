use std::collections::HashMap;
use std::time::Instant;
mod file_utils;

fn part1() {

    // a1 - b1 + a2 - b2
    // a1 + a2 + a3 - (b1 + b2 + b3)
    
    let input = file_utils::get_file_content("./input/day1.txt");


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

    println!("[PART 1] The sum is {:?}", sum);
}

// Approach 1 - Slightly more complex and the code is also probably not good
// The idea is that, the final sum for any particular number will be 
// (number * occurence in the left list * occurence in the right list)

// HashMap stores number as the key, and if the number appears on left, increments the first value of vec
// if on the right, increments the second value of vec

// iterates through the hashmap, adding the multiplication to the sum

fn part2() {
    let input = file_utils::get_file_content("./input/day1.txt");

    let mut sum = 0;
    let mut counter: HashMap<i32, Vec<i32>> = HashMap::new();


    for line in input.split("\r\n") {
        let nums: Vec<i32> = line.split("   ")
        .map(|l| 
            l.parse::<i32>()
            .unwrap())
        .collect();

        counter.entry(nums[1]).or_insert(vec![0, 0])[1] += 1;
        counter.entry(nums[0]).or_insert(vec![0, 0])[0] += 1;
 
    }

    for (num, value) in counter.iter() {
        sum += num * value[0] * value[1];
    }
    

    println!("[PART 2] The similarity score is {:?}", sum);

}

// Approach 2 - seems to be a bit faster and much easier to understand.

// Hashmap stores {number: occurence} pairs while the left vector just parses left num and stores
// After parsing, iterate through left arr, find matching key in counter, multiply, and sum all results.

fn part2_2() {
    let input = file_utils::get_file_content("./input/day1.txt");

    let mut left: Vec<i32> = Vec::new();
    let mut counter: HashMap<i32, i32> = HashMap::new();


    for line in input.split("\r\n") {
        let nums: Vec<i32> = line.split("   ")
        .map(|l| 
            l.parse::<i32>()
            .unwrap())
        .collect();

        left.push(nums[0]);
        *counter.entry(nums[1]).or_insert(0) += 1;
    }


    let score: i32 = left.iter()
        .map(|&num| num * counter.get(&num).unwrap_or(&0))
        .sum();

    println!("[PART 2] The similarity score is {:?}", score);
}

fn main() {
    part1();

    println!("[PART 2] Trying out function using only one hash map.");
    let start = Instant::now();
    part2();
    let duration = start.elapsed();
    println!("The call took {:?} time", duration);

    println!("[PART 2] Trying out function using one hash map and one vector.");
    let start = Instant::now();
    part2_2();
    let duration = start.elapsed();
    println!("The call took {:?} time", duration);
}