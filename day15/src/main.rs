use std::{fs, collections::{HashMap, BTreeMap}};

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
            hash *= 17;
            hash = hash %256;
        }
        sum += hash;
    }

    sum
}

fn part_two(clone: String) -> i32 {
    let mut sum = 0;
    let mut light = vec![Vec::<(&str,usize)>::new(); 256];
    for (i,steps) in clone.split(",").enumerate(){
        let mut hash = 0;
        for c in steps.chars().take_while(|c| c != &'=' && c != &'-'){
            hash += c as u32;
            hash *= 17;
            hash = hash %256;
        }
        if steps.contains('=') {
            let label_focal: Vec<&str> = steps.split('=').collect();
            let label = label_focal[0];
            let focal = label_focal[1].parse::<usize>().unwrap();
            if light[hash as usize].iter().any(|(l,_)| l == &label){
                let p = light[hash as usize].iter().position(|(l,_)| l == &label).unwrap();
                if let Some(l_f) = light[hash as usize].get_mut(p) {
                        *l_f = (label, focal);
                }
            }
            else {
                light[hash as usize].push((label,focal));
            }
        }
        else{
            let label_focal: Vec<&str> = steps.split('-').collect();
            light[hash as usize].retain(|(label,_)| label != &label_focal[0]);
        }
    }

    for (i,l) in light.iter().enumerate() {
        for (ii,(_, focal)) in l.iter().enumerate() {
            sum += (1+i) * (ii+1) * focal;
        }
    }

    sum as i32
}