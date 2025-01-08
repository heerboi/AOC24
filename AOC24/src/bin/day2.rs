mod file_utils;


fn part1() {
    let input = file_utils::get_file_content("./input/day2.txt");
    
    let mut safe_reports = 0;

    for line in input.split("\r\n") {
        let nums: Vec<_> = line.split(" ").map(|l| l.parse::<i32>().unwrap()).collect();

        // Immutable reference to refer to the preceding level
        let mut prev: &i32 = &0;

        // Bool that tracks whether the sequence is increasing or decreasing
        let mut increasing = true;

        // Bool to track whether the given levels are safe
        let mut safe = true;

        for (i, item) in nums.iter().enumerate() {

            // If first index, just set it as the prev and move on
            if i == 0 {
                prev = &item;
                continue;
            }

            // If difference between current and preceding is < 1 or > 3
            if 1 > i32::abs(item - prev) || i32::abs(item - prev) > 3 {
                safe = false;
                break;
            }

            // If increasing order
            if item > prev {

                // If 2nd digit, use it to initiate the increasing boolean
                if i == 1 {
                    increasing = true;
                }

                // If we're further ahead in the sequence, and the sequence changes from
                // decreasing to increasing
                else if !increasing {
                    safe = false;
                    break;
                }

                // if all is good, set current as prev and move on
                prev = &item;
                continue;
            } else if item < prev {

                // If current is less than prev, set the sequence as decreasing
                if i == 1 {
                    increasing = false;
                }

                // If sequence was increasing and now decreases, BREAK!!!
                else if increasing {
                    println!("Suddenly decreasing: {}\n", line);
                    safe = false;
                    break;
                }
                
                prev = &item;
                continue;
            }
        }
        if safe {
            safe_reports += 1;
        }
    }
    println!("[PART 1] The amount of safe reports are {}", safe_reports);
}

fn main() {
    part1();
}