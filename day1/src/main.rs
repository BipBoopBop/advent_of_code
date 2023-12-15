use std::fs;
use regex::Regex;

fn main() {
    
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input: String) -> i32{
    let mut sum = 0;
    
    for line in input.lines(){
        let mut a: String = line.chars().filter(|c| c.is_numeric()).collect();
        if a.len() ==1{
            a += &a.to_string();
        }
        if a.len() >2{
            a = a.chars().nth(0).unwrap().to_string() + a.chars().last().unwrap().to_string().as_str(); 
        }
        sum += a.parse::<i32>().unwrap();
    }

    sum
}


fn part_two(input: String) -> i32{
    let mut sum = 0;
    
    for line in input.lines(){
        let line_reverse = line.chars().rev().collect::<String>();
        
        let re = Regex::new(r"(?:zero|one|two|three|four|five|six|seven|eight|nine)|(?:[0-9])").unwrap();
        let caps: Vec<&str> = re.captures_iter(line).map(|c| {
            let (num, []) = c.extract();
            num
        }).collect();

        let re_reverse = Regex::new(r"(?:orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)|(?:[0-9])").unwrap();
        let  caps_reverse: Vec<&str> = re_reverse.captures_iter(line_reverse.as_str()).map(|c| {
            let (num, []) = c.extract();
            num
        }).collect();
        
        let mut num = String::new();

        for c in [caps[0],caps_reverse[0]]{
            match c{
                "zero" | "orez" => num += "0",
                "one" | "eno" => num += "1",
                "two" | "owt" => num += "2",
                "three" | "eerht" => num += "3",
                "four" | "ruof" => num += "4",
                "five" | "evif" => num += "5",
                "six" | "xis" => num += "6",
                "seven" | "neves" => num += "7",
                "eight" | "thgie" => num += "8",
                "nine" | "enin" => num += "9",
                _ => num += c
            }
        }

        sum += num.parse::<i32>().unwrap();
    }
    
    sum
}