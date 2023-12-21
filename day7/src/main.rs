use std::{fs, borrow::Borrow};
use fancy_regex::Regex;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> i32 {
    let mut sum = 0;
    let input: Vec<(&str, i32)> = input_text
        .lines()
        .map(|line| {
            let l = line.split(" ").collect::<Vec<&str>>() ;
            (l[0],
            l[1].parse().unwrap())
        }).collect();

    let reg_type = [
        Regex::new(r"(\w)\1{4}").unwrap(),
        Regex::new(r"(\w)\1{3}").unwrap(),
        Regex::new(r"(\w)\1\1(\w)\2|(\w)\3(\w)\4\4").unwrap(),
        Regex::new(r"(\w)\1\1").unwrap(),
        Regex::new(r"(\w)\1.*(\w)\2").unwrap(),
        Regex::new(r"(\w)\1").unwrap()
    ];

    let mut hands_type: [Vec<usize>;7] = Default::default() ;

    for (i,(hand, _)) in input.iter().enumerate(){
        let mut hand_sort: Vec<char> = hand.chars().collect();
        hand_sort.sort();
        for (reg_i,regex) in reg_type.iter().enumerate(){
            if regex.is_match(String::from_iter(hand_sort.clone()).as_str()).unwrap(){
                hands_type[reg_type.len() - reg_i].push(i);
                break;
            }
            else if reg_i == reg_type.len()-1{
                hands_type[0].push(i);
            }
        }
    }

    for hand in &mut hands_type{
        if hand.len() <= 1 {continue;}

        hand.sort_by(|a,b| {
            let hand_a = input[*a].0.chars();
            let mut hand_b = input[*b].0.chars();
            for a_c in hand_a{
                let Some(b_c) = hand_b.nth(0) else {panic!("error sorting")}; 
                if a_c == b_c {continue;} 
                else if a_c == 'A' {return std::cmp::Ordering::Greater;}
                else if b_c == 'A' {return std::cmp::Ordering::Less;}
                else if a_c == 'K' {return std::cmp::Ordering::Greater;}
                else if b_c == 'K' {return std::cmp::Ordering::Less;}
                else if a_c == 'Q' {return std::cmp::Ordering::Greater;}
                else if b_c == 'Q' {return std::cmp::Ordering::Less;}
                else if a_c == 'J' {return std::cmp::Ordering::Greater;}
                else if b_c == 'J' {return std::cmp::Ordering::Less;}
                else if a_c == 'T' {return std::cmp::Ordering::Greater;}
                else if b_c == 'T' {return std::cmp::Ordering::Less;}
                else {return a_c.to_digit(10).cmp(&b_c.to_digit(10));}
            }
            std::cmp::Ordering::Equal
        });

    }

    for (i,hand_i) in hands_type.into_iter().flatten().enumerate(){
        let (_, bid) = input[hand_i];
        sum += bid * (i as i32 + 1);
    }
    sum
}

fn part_two(input_text: String) -> i32 {
    let mut sum = 0;
    let input: Vec<(&str, i32, usize)> = input_text
        .lines()
        .map(|line| {
            let l = line.split(" ").collect::<Vec<&str>>() ;
            (l[0],
            l[1].parse().unwrap(),
            l[0].chars().filter(|&c| c == 'J').count())
        }).collect();

    let reg_type = [
        Regex::new(r"(\w)\1{4}").unwrap(),
        Regex::new(r"(\w)\1{3}").unwrap(),
        Regex::new(r"(\w)\1\1(\w)\2|(\w)\3(\w)\4\4").unwrap(),
        Regex::new(r"(\w)\1\1").unwrap(),
        Regex::new(r"(\w)\1.*(\w)\2").unwrap(),
        Regex::new(r"(\w)\1").unwrap()
    ];

    let mut hands_type: [Vec<usize>;7] = Default::default() ;

    for (i,(hand, _, j_num)) in input.iter().enumerate(){
        let mut hand_sort: Vec<char> = hand.chars().collect();
        hand_sort.sort();
        let hand_sorted = String::from_iter(hand_sort);
        for (reg_i,regex) in reg_type.iter().enumerate(){
            if regex.is_match(hand_sorted.as_str()).unwrap(){
                if reg_i == 1 && j_num > &0 {
                    hands_type[reg_type.len() - reg_i + 1].push(i);
                }
                else if reg_i == reg_type.len() -2 && j_num == &2 {
                    hands_type[reg_type.len() - reg_i + 3].push(i);
                }
                else if j_num >= &1 && j_num <= &3{
                    hands_type[reg_type.len() - reg_i + 2].push(i);
                }
                else {
                    hands_type[reg_type.len() - reg_i].push(i);
                }
                break;
            }
            else if reg_i == reg_type.len()-1{
                if j_num == &1 { hands_type[1].push(i); }
                else { hands_type[0].push(i); }
            }
        }
    }

    for hand in &mut hands_type{
        if hand.len() <= 1 {continue;}

        hand.sort_by(|a,b| {
            let hand_a = input[*a].0.chars();
            let mut hand_b = input[*b].0.chars();
            for a_c in hand_a{
                let Some(b_c) = hand_b.nth(0) else {panic!("error sorting")}; 
                if a_c == b_c {continue;} 
                else if a_c == 'A' {return std::cmp::Ordering::Greater;}
                else if b_c == 'A' {return std::cmp::Ordering::Less;}
                else if a_c == 'K' {return std::cmp::Ordering::Greater;}
                else if b_c == 'K' {return std::cmp::Ordering::Less;}
                else if a_c == 'Q' {return std::cmp::Ordering::Greater;}
                else if b_c == 'Q' {return std::cmp::Ordering::Less;}
                else if a_c == 'J' {return std::cmp::Ordering::Less;}
                else if b_c == 'J' {return std::cmp::Ordering::Greater;}
                else if a_c == 'T' {return std::cmp::Ordering::Greater;}
                else if b_c == 'T' {return std::cmp::Ordering::Less;}
                else {return a_c.to_digit(10).cmp(&b_c.to_digit(10));}
            }
            std::cmp::Ordering::Equal
        });

    }

    for (i,hand_i) in hands_type.into_iter().flatten().enumerate(){
        let (_, bid,_) = input[hand_i];
        sum += bid * (i as i32 + 1);
    }
    sum
}
