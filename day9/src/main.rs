use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> i32 {
    let mut sum = 0;
    let inputs: Vec<Vec<i32>> = input_text
        .lines()
        .map(|l| {
            l.split(char::is_whitespace).map(|n| n.parse().unwrap()).collect::<Vec<i32>>()
        }).collect();

    for history in inputs{
        sum += extrapolate1(history.clone());
    }

    sum
}



fn part_two(input_text: String) -> i32 {
    let mut sum = 0;
    let inputs: Vec<Vec<i32>> = input_text
        .lines()
        .map(|l| {
            l.split(char::is_whitespace).map(|n| n.parse().unwrap()).collect::<Vec<i32>>()
        }).collect();

    for history in inputs{
        sum += extrapolate2(history.clone());
    }

    sum
}

fn extrapolate1(history: Vec<i32>) -> i32 {
    if history.iter().all(|n| n==&0) { return 0; }
    
    let mut steps: Vec<i32> = Vec::new();
    for i in 0..history.len()-1{
        steps.push(history[i+1] - history[i]);
    }

    history.last().unwrap() + extrapolate1(steps)
}

fn extrapolate2(history: Vec<i32>) -> i32 {
    if history.iter().all(|n| n==&0) { return 0; }
    
    let mut steps: Vec<i32> = Vec::new();
    for i in 0..history.len()-1{
        steps.push(history[i+1] - history[i]);
    }

    history.first().unwrap() - extrapolate2(steps)
}