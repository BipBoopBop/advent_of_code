use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(clone: String) -> u32 {
    let mut sum = 0;

    for steps in clone.split(","){
        let mut hash = 0;
        for c in steps.chars(){
            hash += c as u32;
            println!("{c} : {hash}");
            hash *= 17;
            hash = hash %256;
        }
        sum += hash;
    }

    sum
}

fn part_two(clone: String) -> i32 {
    0
}