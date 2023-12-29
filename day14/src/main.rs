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
    let line_len = clone.lines().last().unwrap().len();
    let col_len = clone.lines().count();

    let mut input= vec![Vec::<(usize,char)>::new(); line_len]; 

    for (line_i, line) in clone.lines().enumerate(){
        for (i,c) in line.char_indices().filter(|(_,c)| c==&'O' || c==&'#'){
            input[i].push((col_len-line_i-1,c));
        }
    }
    println!("{:?}\n", input);

    for _ in 0..1{//000000000{
        for _ in 0..3{
            input = roll(input);
            input = rotate(input, col_len); //col_len error
        }
    }

    println!("{:?}", input);

    load(input, col_len)
    
}

fn load(input: Vec<Vec<(usize, char)>>, col_len: usize) -> i32 {
    let mut sum = 0;
    for col in input{
        let mut max = col_len-1;
        for (i,c) in col {
                if c == '#' { if i >0 { max = i-1; } }
            else {
                sum +=max;
                if max >0 {
                    max -=1;
                }
            }
        }
    }
    
    sum as i32
}

fn rotate(input: Vec<Vec<(usize, char)>>, col_len: usize) -> Vec<Vec<(usize, char)>> {
    let mut res = vec![Vec::<(usize,char)>::new(); col_len];

    for (col_i,col) in input.into_iter().enumerate(){
        for (i,c) in col {
            res[i].push((col_i,c));
        }
    }

    res
}

fn roll(input: Vec<Vec<(usize, char)>>) -> Vec<Vec<(usize, char)>> {
    let mut res: Vec<Vec<(usize, char)>> = Default::default();
    let len = input.len();

    for col in input{
        let mut v = Vec::<(usize, char)>::new();
        let mut max = len-1;
        for (i,c) in col {
            if c == '#' {
                v.push((i,c)); 
                if i >0 { max = i-1; }
            }
            else {
                v.push((max, c));
                if max >0 { max -=1; }
            }
        }
        res.push(v);
    }

    res
}
