use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> i64 {
    let input: Vec<&str> = input_text.split("\r\n\r\n").collect();

    let seeds: Vec<i64> = input[0].split(": ").last().unwrap().split(" ").map(|num| num.parse::<i64>().unwrap()).collect();

    let list_maps: Vec<Vec<(i64,i64,i64)>> = input
        .iter()
        .skip(1)
        .map(|&maps| {
            maps.split("\r\n").skip(1).map(|str_nums| {
                let nums: Vec<i64> = str_nums.split(" ").map(|num| num.parse::<i64>().unwrap()).collect();
                (nums[0], nums[1], nums[2])
            }).collect::<Vec<(i64,i64,i64)>>()
        }).collect();
    
    let mut mapped_seeds: Vec<(i64,bool)> = seeds.iter().map(|s| (*s, true)).collect();
    for maps in list_maps{
        for (dest, source, range) in maps{
            mapped_seeds = mapped_seeds
                .iter()
                .map(|(s, bool)| {
                    if (source..source+range).contains(s) && *bool{
                        return (dest + s - source, false);
                    }
                    (*s,*bool)
                })
                .collect();
        }
        mapped_seeds = mapped_seeds.iter().map(|(s,_)| (*s,true)).collect();
    }

    mapped_seeds.iter().fold(i64::MAX, |acc, (s,_)| acc.min(*s))
}

fn part_two(input_text: String) -> i128 {
    let input: Vec<&str> = input_text.split("\r\n\r\n").collect();

    let seeds: Vec<i128> = input[0].split(": ").last().unwrap().split(" ").map(|num| num.parse::<i128>().unwrap()).collect();

    let list_maps: Vec<Vec<(i128,i128,i128)>> = input
        .iter()
        .skip(1)
        .map(|&maps| {
            maps.split("\r\n").skip(1).map(|str_nums| {
                let nums: Vec<i128> = str_nums.split(" ").map(|num| num.parse::<i128>().unwrap()).collect();
                (nums[0], nums[1], nums[2])
            }).collect::<Vec<(i128,i128,i128)>>()
        }).collect();
    
    let mut mapped_seeds = Vec::<(i128,bool)>::new();
    let mut i = 0;
    while i < seeds.len()/2{
        mapped_seeds = [mapped_seeds, (seeds[i]..seeds[i]+seeds[i+1]).map(|s| (s, true)).collect()].concat();
        i = i+2;
        println!("{i}");
    }

    println!("end compute mapped_seeds");

    for (i,maps) in list_maps.into_iter().enumerate(){
        println!("maps : {i}");

        mapped_seeds = mapped_seeds.iter().map(|(s,_)| (*s,true)).collect();
        for (map_i, (dest, source, range)) in maps.into_iter().enumerate(){
            if map_i%5 == 0 {println!("range maps : {map_i}")}

            mapped_seeds = mapped_seeds
                .iter()
                .map(|(s, bool)| {
                    if (source..source+range).contains(s) && *bool{
                        return (dest + s - source, false);
                    }
                    (*s,*bool)
                })
                .collect();
        }
        println!("end maps : {i}");
        mapped_seeds = mapped_seeds.into_iter().map(|(s,_)| (s,true)).collect();
    }
    println!("check min");


    mapped_seeds.iter().fold(i128::MAX, |acc, (s,_)| acc.min(*s))
}
