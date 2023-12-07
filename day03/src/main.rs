use std::fs;

fn _read_input1(input: &str) -> (Vec<Vec<Option<u8>>>, Vec<Vec<bool>>){
    let mut lines = input.lines();
    let width = lines.next().expect("at least one line").len();
    let height = lines.count() + 1;
    let mut input_vec: Vec<Vec<Option<u8>>> = Vec::with_capacity(height);
    let mut valid_spots: Vec<Vec<bool>> = Vec::with_capacity(height);
    for _ in 0..height {
        input_vec.push(vec![None;width]);
        valid_spots.push(vec![false;width]);
    }

    for (i, line) in input.lines().enumerate(){
        for (j, character) in line.chars().enumerate(){
            if character.is_digit(10){
                input_vec[i][j] = Some(character.to_digit(10).expect("checked is digit...") as u8);
            } else if character != '.' {
                for k in 0..3{
                    for l in 0..3{
                        valid_spots[(i+k-1).clamp(0, height-1)][(j+l-1).clamp(0, width-1)] = true;
                    }
                }
            }
        }
    }
    (input_vec, valid_spots)
}

fn _part1(input: &str) -> u64{
    let (digits, valid_spots) = _read_input1(input);
    let mut sum = 0;
    for i in 0..digits.len(){
        let mut x:u64 = 0;
        let mut valid = false;
        for j in 0..digits[0].len(){
            match digits[i][j] {
                Some(digit) =>  {x = x*10 + digit as u64; valid |= valid_spots[i][j]},
                None if valid => {sum += x; x = 0; valid=false;},
                _ => x = 0
            }
        }
        if valid {
            sum += x;
        }
    }
    sum
}

#[derive(Clone)]
enum CogState{
    Empty,
    Single(u64),
    Full(u64, u64),
    Over
}

fn update_cogs(cogs: &mut Vec<Vec<CogState>>, i: usize, j: usize, length: usize, x:u64){
    for k in 0..3{
        for l in 0..length+2{
            cogs[i+k][j-length+l] = match cogs[i+k][j-length+l] {
                CogState::Empty => CogState::Single(x),
                CogState::Single(u) => CogState::Full(u, x),
                CogState::Full(_,_) => CogState::Over,
                _ => CogState::Over
            };
        }
    }
}

fn read_input2(input: &str) -> Vec<Vec<CogState>>{
    let mut lines = input.lines();
    let width = lines.next().expect("at least one line").len();
    let height = lines.count() + 1;
    let mut cogs: Vec<Vec<CogState>> = Vec::with_capacity(height+2);
    for _ in 0..height+2 {
        cogs.push(vec![CogState::Empty;width+2]);
    }

    for (i, line) in input.lines().enumerate(){
        let mut x: u64 = 0;
        let mut length = 0;
        for (j, character) in line.chars().enumerate(){
            if character.is_digit(10){
                x = x*10 + character.to_digit(10).expect("checked is digit") as u64;
                length += 1;
            } else {
                if x != 0 {
                    update_cogs(&mut cogs, i, j, length, x);
                }
                
                x = 0;
                length = 0;
            }
            if character != '*'{
                cogs[i+1][j+1] = CogState::Over;
            }
        }
        if x != 0{
            update_cogs(&mut cogs, i, width-2, length, x)
        }
    }
    cogs
}

fn part2(input: &str) -> u64 {
    let cogs = read_input2(input);
    let mut sum = 0;
    for i in 1..cogs.len()-1 {
        for j in 1..cogs.len()-1 {
            match cogs[i][j]{
                CogState::Full(x, y) => sum += x*y,
                _ => ()
            }
        }
    }
    sum
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).expect("input file required")))
}
