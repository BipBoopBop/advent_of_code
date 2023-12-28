use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let part_one_result = part_one(input.clone());
    let part_two_result = part_two(input.clone());

    println!("part_one_result : {part_one_result}");
    println!("part_two_result : {part_two_result}");
}

fn part_one(clone: String) -> usize {
    let mut sum = 0;
    for input_text in clone.split("\r\n\r\n"){
        let rows: Vec<_> = input_text.lines().map(|l| l.to_string()).collect();
        let row_len = rows[0].len();
        let mut columns: Vec<String> = vec!["".to_string(); row_len];
        for (i,c) in input_text.replace("\r\n", "").char_indices() {
            columns[i%row_len] += &c.to_string();
        }


        let mut mirror: Vec<usize> = rows
            .iter()
            .enumerate()
            .filter_map(|(i,l)| if Some(l) == rows.first() { Some(i+1)} else {None})
            .collect();

        if mirror.len() < 2 {
            mirror = rows
            .iter()
            .enumerate()
            .filter_map(|(i,l)| if Some(l) == rows.last() { Some(i+1)} else {None})
            .collect();
        }

        if mirror.len() >= 2 && is_symetric(mirror.clone(), rows){ 
            sum += 100 * (mirror[0] + (mirror[1]-mirror[0])/2);
        }

        else {
            mirror = columns
            .iter()
            .enumerate()
            .filter_map(|(i,l)| if Some(l) == columns.first() { Some(i+1)} else {None})
            .collect();
            
            if mirror.len() < 2 {
                mirror = columns
                .iter()
                .enumerate()
                .filter_map(|(i,l)| if Some(l) == columns.last() { Some(i+1)} else {None})
                .collect();
            }

            if mirror.len() >= 2 && is_symetric(mirror.clone(), columns){ 
                sum += mirror[0] + (mirror[1]-mirror[0])/2;

            }
        }
    }

    sum
}

fn is_symetric(mirror: Vec<usize>, columns: Vec<String>) -> bool {
    let len = (mirror[1]-mirror[0])/2;
    for i in 0..=len {
        if columns[mirror[0]+i-1] != columns[mirror[1]-i-1]{
            return false;
        }
    }

    true
}

fn part_two(clone: String) -> i32 {

    0
}
