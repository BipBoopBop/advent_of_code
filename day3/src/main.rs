use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let mut sum = 0;

    let line_len = input.lines().last().unwrap().len() as i32;
    let mut input_arr: Vec<(usize, char)> = input
        .char_indices().filter(|c| 
            c.1 != '.' && !c.1.is_whitespace()
        )
        .collect();
    
    let adjacent_digits: Vec<(usize, char)>  = input_arr.clone()
        .into_iter()
        .filter(|(_,c)| !c.is_numeric())
        .flat_map(|(symbol_i, _)| input_arr.clone()
            .into_iter()
            .filter(move |(i,c)|
                [
                 (symbol_i as i32)-line_len-3, 
                 (symbol_i as i32)-line_len-2, 
                 (symbol_i as i32)-line_len-1,
                 (symbol_i as i32)-1, 
                 (symbol_i as i32)+1, 
                 (symbol_i as i32)+line_len+1,
                 (symbol_i as i32)+line_len+2, 
                 (symbol_i as i32)+line_len+3
                 ]
                .contains(&(*i as i32))
                && c.is_numeric()
            )
        ).collect();

    // println!("{:?}",adjacent_digits);
    for (digit_i, _) in adjacent_digits{

        let mut nums = Vec::<(usize,char)>::new();

        if input_arr.iter().any(|(i,c)| c.is_numeric() && *i == digit_i){
            nums.push(*input_arr.iter().find(|(i,c)| c.is_numeric() && *i == digit_i).unwrap());
            
            if input_arr.iter().any(|(i,c)| c.is_numeric() && *i == digit_i+1){
                nums.push(*input_arr.iter().find(|(i,c)| c.is_numeric() && *i == digit_i+1).unwrap());
                
                if input_arr.iter().any(|(i,c)| c.is_numeric() && *i == digit_i+2){
                    nums.push(*input_arr.iter().find(|(i,c)| c.is_numeric() && *i == digit_i+2).unwrap());
                }
                
                else if input_arr.iter().any(|(i,c)| c.is_numeric() && *i == digit_i-1){
                    nums.push(*input_arr.iter().find(|(i,c)| c.is_numeric() && *i == digit_i-1).unwrap());
                }
            }
            
            else if input_arr.iter().any(|(i,c)| c.is_numeric() && *i == digit_i-1){
                nums.push(*input_arr.iter().find(|(i,c)| c.is_numeric() && *i == digit_i-1).unwrap());
                
                if input_arr.iter().any(|(i,c)| c.is_numeric() && *i == digit_i-2){
                    nums.push(*input_arr.iter().find(|(i,c)| c.is_numeric() && *i == digit_i-2).unwrap());
                }
            }
        }

        if nums.is_empty(){ continue;}
        
        for &num in nums.iter(){
            let input_arr_i = input_arr.clone().into_iter().position(|ci| ci == num).unwrap();
            let _ = std::mem::replace(&mut input_arr[input_arr_i], (num.0,'.'));
        }
        
        nums.sort_by(|a, b| a.0.cmp(&b.0));
        
        sum += nums.iter().map(|(_,c)| c).collect::<String>().parse::<i32>().unwrap();
    }
    
    println!("{sum}");
}
