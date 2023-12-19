use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> u32 {
    let mut sum = 1;
    let input: Vec<&str> = input_text.split("\r\n").collect();
    let times: Vec<u32> = input[0]
        .split(" ")
        .flat_map(|s| s.parse())
        .collect();
    let distances_rec: Vec<u32> = input[1]
        .split(" ")
        .flat_map(|s| s.parse())
        .collect();

    for (i,time) in times.iter().enumerate(){
        let mut hold_time = *time;
        let mut distance = 0;

        while distance <= distances_rec[i]{
            hold_time -= 1;
            distance = hold_time * (time - hold_time);
        }

        let mut ways_num = hold_time;
        hold_time = 0;
        distance = 0;
        
        while distance <= distances_rec[i]{
            hold_time += 1;
            distance = hold_time * (time - hold_time);
        }

        ways_num -= hold_time;
        sum *= ways_num+1;
    }
    sum
}

fn part_two(input_text: String) -> u64 {
    let mut sum = 1;
    let input: Vec<&str> = input_text.split("\r\n").collect();
    let times: Vec<u32> = input[0]
        .split(" ")
        .flat_map(|s| s.parse())
        .collect();
    let distances_rec: Vec<u32> = input[1]
        .split(" ")
        .flat_map(|s| s.parse())
        .collect();
    let time: u64 = times.into_iter().map(|n| n.to_string()).collect::<String>().parse().unwrap();
    let distance_rec: u64 = distances_rec.into_iter().map(|n| n.to_string()).collect::<String>().parse().unwrap();

    let mut hold_time = time;
    let mut distance = 0;

    while distance <= distance_rec{
        hold_time -= 1;
        distance = hold_time * (time - hold_time);
    }

    let mut ways_num = hold_time;
    hold_time = 0;
    distance = 0;
    
    while distance <= distance_rec{
        hold_time += 1;
        distance = hold_time * (time - hold_time);
    }

    ways_num -= hold_time;
    sum *= ways_num+1;
    sum
}
