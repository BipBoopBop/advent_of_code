use std::{fs, collections::LinkedList};

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> usize {
    let input: Vec<(usize, char)> = input_text.replace("\r\n", "").char_indices().filter(|(_,c)| c != &'.' && !"\r\n".contains(*c)).collect();

    let line_len = input_text.lines().last().unwrap().len();
    let start = input.clone().into_iter().find(|(_, c)| *c == 'S').unwrap();
    let start_nexts: Vec<(usize, char)> = input.clone().into_iter().filter(|(i,c)| {
        i == &(start.0-1) && (c == &'-' || c == &'F' || c == &'L') ||
        i == &(start.0+1) && (c == &'-' || c == &'7' || c == &'J') ||
        i == &(start.0-line_len) && (c == &'|' || c == &'F' || c == &'7') ||
        i == &(start.0+line_len) && (c == &'|' || c == &'L' || c == &'J')
    }).collect();

    let mut previous_pipe = [start, start];

    let mut linked_pipes: LinkedList<(usize, char)> = LinkedList::new();
    linked_pipes.push_front(start_nexts[0]);
    linked_pipes.push_back(start_nexts[1]);

    while linked_pipes.front() != linked_pipes.back(){
        let lp_clone = linked_pipes.clone();
        let mut current_pipe = lp_clone.front();
        if let Some((i,c)) = current_pipe {
            match c {
                '|' => {
                    linked_pipes.push_front(*input.iter().find(|(next_i, _)| (*next_i == i+line_len && *next_i != previous_pipe[0].0) || (*next_i == i-line_len && *next_i != previous_pipe[0].0)).unwrap());
                },
                '-' => {
                    linked_pipes.push_front(*input.iter().find(|(next_i, _)| (*next_i == i+1 && *next_i != previous_pipe[0].0) || (*next_i == i-1 && *next_i != previous_pipe[0].0)).unwrap());
                },
                'L' => {
                    linked_pipes.push_front(*input.iter().find(|(next_i, _)| (*next_i == i+1 && *next_i != previous_pipe[0].0) || (*next_i == i-line_len && *next_i != previous_pipe[0].0)).unwrap());
                },
                'J' => {
                    linked_pipes.push_front(*input.iter().find(|(next_i, _)| (*next_i == i-1 && *next_i != previous_pipe[0].0) || (*next_i == i-line_len && *next_i != previous_pipe[0].0)).unwrap());
                },
                '7' => {
                    linked_pipes.push_front(*input.iter().find(|(next_i, _)| (*next_i == i+line_len && *next_i != previous_pipe[0].0) || (*next_i == i-1 && *next_i != previous_pipe[0].0)).unwrap());
                },
                'F' => {
                    linked_pipes.push_front(*input.iter().find(|(next_i, _)| (*next_i == i+line_len && *next_i != previous_pipe[0].0) || (*next_i == i+1 && *next_i != previous_pipe[0].0)).unwrap());
                },
                _ => {}
            }
            previous_pipe[0] = *current_pipe.unwrap();
        }

        current_pipe = lp_clone.back();
        if let Some((i,c)) = current_pipe {
            match c {
                '|' => {
                    linked_pipes.push_back(*input.iter().find(|(next_i, _)| (*next_i == i+line_len && *next_i != previous_pipe[1].0) || (*next_i == i-line_len && *next_i != previous_pipe[1].0)).unwrap());
                },
                '-' => {
                    linked_pipes.push_back(*input.iter().find(|(next_i, _)| (*next_i == i+1 && *next_i != previous_pipe[1].0) || (*next_i == i-1 && *next_i != previous_pipe[1].0)).unwrap());
                },
                'L' => {
                    linked_pipes.push_back(*input.iter().find(|(next_i, _)| (*next_i == i+1 && *next_i != previous_pipe[1].0) || (*next_i == i-line_len && *next_i != previous_pipe[1].0)).unwrap());
                },
                'J' => {
                    linked_pipes.push_back(*input.iter().find(|(next_i, _)| (*next_i == i-1 && *next_i != previous_pipe[1].0) || (*next_i == i-line_len && *next_i != previous_pipe[1].0)).unwrap());
                },
                '7' => {
                    linked_pipes.push_back(*input.iter().find(|(next_i, _)| (*next_i == i+line_len && *next_i != previous_pipe[1].0) || (*next_i == i-1 && *next_i != previous_pipe[1].0)).unwrap());
                },
                'F' => {
                    linked_pipes.push_back(*input.iter().find(|(next_i, _)| (*next_i == i+line_len && *next_i != previous_pipe[1].0) || (*next_i == i+1 && *next_i != previous_pipe[1].0)).unwrap());
                },
                _ => {}
            }
            previous_pipe[1] = *current_pipe.unwrap();
        }
    }

    linked_pipes.len()/2
}


fn part_two(input_text: String) -> i32 {
    0
}