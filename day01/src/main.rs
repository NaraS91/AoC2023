use std::fs;
use std::str::Chars;

fn _part1(input: &str) -> u32{
    let mut sum: u32 = 0;
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap();
        let second = line.chars().rev().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap();
        sum += first * 10 + second;
    }
    sum
}

fn find_digit(valid_digits: &[&[u8];9], line: &Chars) -> u32{
    let mut common: [usize; 9] = [0; 9];
    let mut found_digit: i32 = -1;
    let mut cs: [u8;1] = [0];
    for character in line.clone(){
        character.encode_utf8(&mut cs);
        let c = cs[0];
        for i in 0..9{
            if c == valid_digits[i][common[i]] {
                common[i] += 1;
                if common[i] == valid_digits[i].len() {
                    found_digit = (i + 1) as i32;
                }
            } else {
                common[i] = if c == valid_digits[i][0] {
                    1 // case of ff-ive, ss-even etc
                } else if i == 8 && common[i] == 3 && c == b'i' {
                    2 //case of nin-ine, expecting e, got i
                }  else {
                    0
                }
            }
        }
        if character.is_digit(10) {
            found_digit = character.to_digit(10).unwrap() as i32;
        }

        if found_digit >= 0 {
            break;
        }
    }
    found_digit as u32
}

fn part2(input: &str) -> u32{
    let valid_digits: [&[u8];9] = ["one".as_bytes(), "two".as_bytes(), "three".as_bytes(), "four".as_bytes(), "five".as_bytes(), "six".as_bytes(), "seven".as_bytes(), "eight".as_bytes(), "nine".as_bytes()];
    let reversed_valid_digits: [&[u8];9] = ["eno".as_bytes(), "owt".as_bytes(), "eerht".as_bytes(), "ruof".as_bytes(), "evif".as_bytes(), "xis".as_bytes(), "neves".as_bytes(), "thgie".as_bytes(), "enin".as_bytes()];
    let mut sum = 0;
    for line in input.lines(){
        let line_b = line.chars();
        let first_digit: u32 = find_digit(&valid_digits, &line_b);
        let reversed_line: String = line.chars().rev().collect();
        let last_digit: u32 = find_digit(&reversed_valid_digits, &reversed_line.chars());
        sum += first_digit * 10 + last_digit;
    }
    sum
}

fn main() {
    let path:&str  = "./input.txt";
    println!("solution: {}", part2(&fs::read_to_string(path).unwrap()));
}
