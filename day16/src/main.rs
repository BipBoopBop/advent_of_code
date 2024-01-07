use std::{fs, cmp};

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(clone: String) -> i32 {
    let len = clone.lines().count();
    let tiles: Vec<([usize; 2], char)> = clone.replace("\r\n", "")
        .char_indices()
        .filter_map(|(i,c)| {
            if c == '|' || c == '-' || c == '/' || c == '\\' {
                Some(([i%len, i/len], c))
            }
            else { None }
        }).collect();


    let start: ([usize; 2], Direction) = ([0,0], Direction::Down);
    let mut seen = Vec::<[usize;2]>::new();

    energize(start, &tiles, Vec::<([usize; 2], Direction)>::new(), &mut seen, &len) as i32
}

fn energize(start: ([usize; 2], Direction), tiles: &[([usize; 2], char)], history: Vec<([usize; 2], Direction)>, seen: &mut Vec::<[usize; 2]>, len: &usize) -> i32 {
    if history.contains(&start) {
        return 0;
    }
    let mut next_history = history;
    next_history.push(start.clone());

    let next_tile = match start.1 {
        Direction::Right => {
            tiles.iter()
                .find_map(|([x,y], c)| {
                    if x > &start.0[0] && y == &start.0[1] && c != &'-' { Some(([*x,*y], *c)) } 
                    else { None }
                })
        }
        Direction::Left => {
            tiles.iter()
                .rfind(|([x,y], c)| x < &start.0[0] && y == &start.0[1] && c != &'-' )
                .map(|&tile| tile)
        }
        Direction::Up => {
            tiles.iter()
                .rfind(|([x,y], c)| y < &start.0[1]  && x == &start.0[0] && c != &'|')
                .map(|&tile| tile)
        }
        Direction::Down => {
            tiles.iter()
                .find_map(|([x,y], c)| {
                    if y > &start.0[1] && x == &start.0[0] && c != &'|' { Some(([*x,*y], *c)) }
                    else { None }
                })
        }
    };


    if let Some(([x,y], c)) = next_tile {
        let mut energized = (x as i32- start.0[0] as i32).abs() + (y as i32 - start.0[1] as i32).abs();// + next_history.iter().any(|(p,_)| p == &[x,y]) as i32;

        
        match start.1{
            Direction::Right => {
                for xi in start.0[0]..x{
                    let p = [xi,y];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
            Direction::Left => {
                for xi in x+1..=start.0[0]{
                    let p = [xi,y];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
            Direction::Up => {
                for yi in y+1..=start.0[1]{
                    let p = [x,yi];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
            Direction::Down => {
                for yi in start.0[1]..y{
                    let p = [x,yi];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
        }
        match c {
            
            '|' => {
                let mut up_next_history = next_history.clone();
                up_next_history.push(([x,y], Direction::Down));
                let mut down_next_history = next_history.clone();
                down_next_history.push(([x,y], Direction::Up));

                return energized + energize(([x,y], Direction::Up), tiles, up_next_history, seen, len) + energize(([x,y], Direction::Down), tiles, down_next_history, seen, len);
            }
            '-' => {
                let mut left_next_history = next_history.clone();
                left_next_history.push(([x,y], Direction::Right));
                let mut right_next_history = next_history.clone();
                right_next_history.push(([x,y], Direction::Left));

                return energized + energize(([x,y], Direction::Left), tiles, left_next_history.clone(), seen, len) + energize(([x,y], Direction::Right), tiles, right_next_history, seen, len);
            }
            '/' => {
                let dir = match start.1 {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left
                };

                return energized + energize(([x,y], dir), tiles, next_history, seen, len);
            }
            '\\' => {
                let dir = match start.1 {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right
                };

                return energized + energize(([x,y], dir), tiles, next_history, seen, len);
            }
            _ => {  return energized; }
        }
    }
    else {
        let [x,y] = match start.1 {
            Direction::Right => [len-1, start.0[1]],
            Direction::Left => [0,start.0[1]],
            Direction::Up =>  [start.0[0],0],
            Direction::Down =>  [start.0[0],len-1],
        };

        let mut energized = (x as i32- start.0[0] as i32).abs() + (y as i32 - start.0[1] as i32).abs() + 1;

        match start.1{
            Direction::Right => {
                for xi in start.0[0]..=x{
                    let p = [xi,y];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
            Direction::Left => {
                for xi in x..=start.0[0]{
                    let p = [xi,y];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
            Direction::Up => {
                for yi in y..=start.0[1]{
                    let p = [x,yi];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
            Direction::Down => {
                for yi in start.0[1]..=y{
                    let p = [x,yi];
                    if seen.contains(&p) { energized -= 1; }
                    else { seen.push(p); }
                }
            }
        }

        energized
    }

}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}



fn part_two(clone: String) -> i32 {
    0
}
