use std::{fs};

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut sum = 0;

    for line in input.lines(){
        let mut highest_red =0;
        let mut highest_green =0;
        let mut highest_blue =0;

        let line_split = line.split(":").collect::<Vec<&str>>();
        let game_id = line_split[0]
            .chars().filter(|c| c.is_numeric()).collect::<String>()
            .parse::<i32>().unwrap();

        for draw in line_split[1].split(";"){
            for set in draw.split(","){
                if set.contains("red"){
                    let current_red: i32 = set.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
                    highest_red = if highest_red < current_red { current_red } else { highest_red }
                }
                else if set.contains("green"){
                    let current_green: i32 = set.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
                    highest_green = if highest_green < current_green { current_green } else { highest_green }
                }
                else if set.contains("blue"){
                    let current_blue: i32 = set.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
                    highest_blue = if highest_blue < current_blue { current_blue } else { highest_blue }
                }
            }
        }

        if highest_red <= red && highest_green <= green && highest_blue <= blue{
            sum += game_id;
        }
    }

    println!("{sum}");
}
