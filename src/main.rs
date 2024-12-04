use std::env;
use std::fs;
use regex::Regex;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    
    let mut part1_sum = 0;
    let mut part2_sum = 0;

    let mut do_it = true;
    
    for line in lines {
        let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        for m in matches {
            if m.contains("mul") {
                part1_sum = part1_sum + get_sum(m);
            }

            if m == "do()" {
                do_it = true;
            }

            if m == "don't()" {
                do_it = false;
            }

            if do_it && m.contains("mul") {
                part2_sum = part2_sum + get_sum(m);
            }
        }
    }
    println!("sum part 1: {}", part1_sum);
    println!("sum part 2: {}", part2_sum);
}

fn get_sum(m: &str) -> i32 {
    let striped = m.replace("mul(", "").replace(")", "");
    let numbers = striped.split(",").collect::<Vec<&str>>();
    let left = numbers[0].parse::<i32>().unwrap();
    let right = numbers[1].parse::<i32>().unwrap();
    return left * right;
}
