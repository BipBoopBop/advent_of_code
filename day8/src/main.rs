use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> i32 {
    let mut steps = 0;
    let inputs: Vec<&str> = input_text.split("\r\n\r\n").collect();
    let instruction: Vec<usize> = inputs[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else{ 1 })
        .collect();
    let network : HashMap<&str, [&str;2]> = inputs[1]
        .lines()
        .map(|line| {
            let l: Vec<&str> =line.split(" = ").collect();
            let left_right: Vec<&str> = l[1][1..l[1].len()-1].split(", ").collect();
            (l[0], [left_right[0], left_right[1]])
        }).collect();


    let mut node = "AAA";
    let mut instruction_iter = instruction.iter().cycle();
    while node != "ZZZ" {
        steps += 1;
        node = network[node].get(*instruction_iter.next().unwrap()).unwrap();
    }
    

    steps
}

fn part_two(input_text: String) -> u64 {
    let mut steps: Vec<u64> = Vec::new();
    let inputs: Vec<&str> = input_text.split("\r\n\r\n").collect();
    let instruction: Vec<usize> = inputs[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else{ 1 })
        .collect();
    let network : HashMap<&str, [&str;2]> = inputs[1]
        .lines()
        .map(|line| {
            let l: Vec<&str> =line.split(" = ").collect();
            let left_right: Vec<&str> = l[1][1..l[1].len()-1].split(", ").collect();
            (l[0], [left_right[0], left_right[1]])
        }).collect();

    let nodes: Vec<&str> = network
        .iter()
        .flat_map(|n| {
            if n.0.ends_with("A") { Some(*n.0) }
            else { None }
        }).collect();

    for node in nodes {
        let mut instruction_iter = instruction.iter().cycle();
        let mut n = node;
        let mut s = 0;

        while !n.ends_with("Z") {
            s += 1;
            let next_instruction = *instruction_iter.next().unwrap();
            n = network[n].get(next_instruction).unwrap();
        }
        steps.push(s);
    }

    lcm(steps)
}

//must be optimized 
/// Least common multiple vec of numbers
fn lcm(numbers: Vec<u64>) -> u64 {
    let mut temp = numbers.clone();
    
    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).map(|(index, _)| index) {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            },
            None => panic!("Not possible")
        }
    }
}