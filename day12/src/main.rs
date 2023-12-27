use std::{fs, time::Instant};

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> i32 {
    let time = Instant::now();
    let mut sum = 0;
    for line in input_text.lines(){
        let input: Vec<&str> = line.split(" ").collect();
        let springs = input[0];
        let damaged_groups: Vec<usize> = input[1].split(",").filter_map(|s| s.parse().ok()).collect();
        
        sum += arrangement(springs, &damaged_groups);
        
    } 
    
    println!("Elapsed time: {:.2?}", time.elapsed());
    sum
}

fn arrangement(springs: &str, damaged_groups: &Vec<usize>) -> i32 {
    println!("{springs}");
    println!("{:?}", damaged_groups);
    let mut groups: Vec<&str> = springs.split(".").collect();
    groups.retain(|s| !s.is_empty());
    if !springs.contains("?"){
        return (groups.iter().map(|s| s.len()).collect::<Vec<usize>>() == *damaged_groups) as i32;
    }

    let unknown = springs.split("?").count();
    let damaged = springs.split("#").count();
    if damaged+unknown < damaged_groups.iter().sum(){
        return 0;
    }

    arrangement(springs.replacen("?", "#", 1).as_str(), damaged_groups) + arrangement(springs.replacen("?", ".", 1).as_str(), damaged_groups)
}

fn part_two(input_text: String) -> i32 {
    let time = Instant::now();
    let mut sum = 0;
    for line in input_text.lines(){
        let input: Vec<&str> = line.split(" ").collect();
        let springs = input[0].repeat(5);
        let damaged_groups: Vec<usize> = input[1].split(",").filter_map(|s| s.parse().ok()).collect();
        let len =damaged_groups.len();
        
        sum += arrangement(springs.as_str(), &damaged_groups.into_iter().cycle().take(len *5).collect());
        break;
    } 
    
    println!("Elapsed time: {:.2?}", time.elapsed());
    sum
}