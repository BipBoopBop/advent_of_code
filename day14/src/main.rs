use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(clone: String) -> i32 {
    let mut sum = 0;
    let line_len = clone.lines().last().unwrap().len();
    let col_len = clone.lines().count();

    let mut input= vec![Vec::<(usize,char)>::new(); line_len]; 

    for (line_i, line) in clone.lines().enumerate(){
        for (i,c) in line.char_indices().filter(|(_,c)| c==&'O' || c==&'#'){
            input[i].push((col_len-line_i,c));
        }
    }

    for col in input{
        let mut max = col_len;
        for (i,c) in col {
            if c == '#' { max = i-1}
            else {
                sum +=max;
                max -=1;
            }
        }
    }
    
    sum as i32
}

fn part_two(clone: String) -> i32 {
    0
}
