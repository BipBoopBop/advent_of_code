use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let mut sum = 0;

    for line in input.lines(){
        let split: Vec<&str> = line.split("|").collect();
        let winning_nums: Vec<i32> = split[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let nums: Vec<i32> = split[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        sum += nums.iter().fold(0, |acc, x| {
            if winning_nums.contains(x){
                if acc == 0 { acc + 1}
                else { acc * 2}
            }
            else {acc +0}
        });

    }

    println!("{sum}");
}
