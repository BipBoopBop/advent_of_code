use std::{fs,io};

fn main() -> io::Result<()>{
    
    let input = fs::read_to_string("./data/input.txt")?;
    let mut sum = 0;
    
    for line in input.lines(){
        let mut a: String = line.chars().filter(|c| c.is_numeric()).collect();
        if a.len() ==1{
            a += &a.to_string();
        }
        if a.len() >2{
            a = a.chars().nth(0).unwrap().to_string() + a.chars().last().unwrap().to_string().as_str(); 
        }
        println!("{:?}",a);
        sum += a.parse::<i32>().unwrap();
    }

    println!("{sum}");
    Ok(())
}
