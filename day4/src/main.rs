use std::fs;

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
        let split: Vec<&str> = line.split("|").collect();
        let winning_nums: Vec<i32> = split[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let nums: Vec<i32> = split[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        sum += nums.iter().fold(0, |acc, x| {
            if winning_nums.contains(x){
                if acc == 0 { acc + 1}
                else { acc * 2}
            }
            else {acc +0}
        });

    }

    sum
}

fn part_two(input: String) -> usize{
    let mut arr = Vec::<Vec<usize>>::new();

    for line in input.lines(){
        let split: Vec<&str> = line.split("|").collect();
        let winning_nums: Vec<i32> = split[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let nums: Vec<i32> = split[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let sum =nums.iter().fold(0, |acc, x| if winning_nums.contains(x) { acc + 1 } else { acc + 0 });
        arr.push(vec![1;sum]);
    }

    for i in 0..arr.len(){
        if arr[i].is_empty() {continue;}

        for card_i in 0..arr[i].len(){
            let point = arr[i][card_i];
            arr[i+card_i+1].iter_mut().for_each(|c| *c+=point) 
        }
    }
    arr.iter().map(|card| card.iter().sum::<usize>()).sum::<usize>() + arr.len()
}