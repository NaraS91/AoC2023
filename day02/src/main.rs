use std::fs;
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Color{
    fn check_color(color: &str) -> Color{
        match color.chars().next() {
            Some('r') => Color::Red,
            Some('g') => Color::Green,
            Some('b') => Color::Blue,
            _ => panic!("not a color smh")
        }
    }
}

fn part1(input: &str, limits: [u32; 3]) -> u32{
    let mut sum = 0;
    'game_loop: for (i, line) in input.lines().enumerate(){
        let hands = line[line.find(":").unwrap()+1..].split(";");
        for hand in hands{
            let words = hand.split_whitespace();
            let mut last_num = 0;
            for (j, word) in words.enumerate(){
                if j % 2 == 0 {
                    last_num = word.parse().unwrap();
                } else if last_num > limits[Color::check_color(word) as usize] {
                    continue 'game_loop;
                }
            }
        }
        sum += i+1;
    }
    sum as u32
}

fn part2(input: &str) -> u32{
    let mut sum = 0;
    for line in input.lines(){
        let hands = line[line.find(":").unwrap()+1..].split(";");
        let mut cubes = [0,0,0];
        for hand in hands{
            let words = hand.split_whitespace();
            let mut last_num = 0;
            for (j, word) in words.enumerate(){
                if j % 2 == 0 {
                    last_num = word.parse().unwrap();
                } else if last_num > cubes[Color::check_color(word) as usize] {
                    cubes[Color::check_color(word) as usize] = last_num;
                }
            }
        }
        sum += cubes[0] * cubes[1] * cubes[2];
    }
    sum as u32
}

fn main() {
    let path = "./input.txt";
    let limits = [12,13,14]; 
    println!("{}", part2(&fs::read_to_string(path).unwrap()));
}
