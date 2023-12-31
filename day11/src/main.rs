use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(input_text: String) -> i32 {
    let mut sum = 0;
    let mut input: Vec<Vec<char>> = Default::default();

    for s in input_text.lines() {
        let mut line = Vec::<char>::new();
        for c in s.chars() {
            line.push(c)
        }
        input.push(line.clone());
        if s.chars().all(|c| c=='.') {
            input.push(line.clone());
        }
    }

    let mut empty_col: Vec<usize> = Vec::new();
    
    for i in 0..input[0].len() {
        let mut count = 0;
        for ii in 0..input.len(){
            if input[ii][i] == '.' { count += 1 }
        }
        if count == input.len() { empty_col.push(i) }
    }

    for (i,col_i) in empty_col.iter().enumerate() {
        for input_i in 0..input.len() {
            input[input_i].insert(col_i+i, '.');
        }
    }

    let mut galaxies: Vec<(usize,usize)> = Vec::new();

    for i in 0..input.len() {
        for ii in 0..input[0].len() {
            if input[i][ii] == '#' { galaxies.push((i,ii)); }
        }
    }

    while let Some((g_r,g_c)) = galaxies.pop() {
        for (r,c) in &galaxies{
            sum += (g_r as i32 - *r as i32).abs() + (g_c as i32 - *c as i32).abs()
        }
    }

    
    sum
}

fn part_two(input_text: String) -> i64 {
    let mut sum = 0;
    let mut input: Vec<Vec<char>> = Default::default();

    let mut empty_row: Vec<usize> = Vec::new();

    for (i,s) in input_text.lines().enumerate() {
        let mut line = Vec::<char>::new();
        for c in s.chars() {
            line.push(c)
        }
        input.push(line);
        if s.chars().all(|c| c=='.') {
            empty_row.push(i);
        }
    }

    let mut empty_col: Vec<usize> = Vec::new();
    
    for i in 0..input[0].len() {
        let mut count = 0;
        for ii in 0..input.len(){
            if input[ii][i] == '.' { count += 1 }
        }
        if count == input.len() { empty_col.push(i) }
    }

    // for l in &input {
    //     for c in l {
    //         print!("{c}, ")
    //     }
    //     println!();
    // }
    // println!();

    let mut galaxies: Vec<(usize,usize)> = Vec::new();

    for i in 0..input.len() {
        for ii in 0..input[0].len() {
            if input[i][ii] == '#' { galaxies.push((i,ii)); }
        }
    }
    
    while let Some((g_r,g_c)) = galaxies.pop() {
        let empty_g_row = empty_row.iter().fold(0, |acc, i| if g_r > *i { acc+1 } else {acc + 0});
        let empty_g_col = empty_col.iter().fold(0, |acc, i| if g_c > *i { acc+1 } else {acc + 0});
        for (r,c) in &galaxies{
            let empty_r = empty_row.iter().fold(0, |acc, i| if r > i { acc+1 } else {acc + 0});
            let empty_c = empty_col.iter().fold(0, |acc, i| if c > i { acc+1 } else {acc + 0});
            sum += ((g_r as i64 + 1000000 * empty_g_row - empty_g_row) as i64 
                 - (*r as i64 + 1000000 * empty_r - empty_r) as i64)
                 .abs()
                + ((g_c as i64 + 1000000 * empty_g_col - empty_g_col ) as i64
                 - (*c as i64 + 1000000 * empty_c - empty_c) as i64)
                 .abs()
        }
    }

    
    sum
}