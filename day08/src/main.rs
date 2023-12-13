use std::{collections::HashMap, fs};

fn _part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let steps = lines.next().unwrap().as_bytes();
    lines.next();
    let graph: HashMap<String, [String; 2]> = lines.map(|line| {
        let mut words = line.split_whitespace();
        (words.next().unwrap().to_owned(), [words.nth(1).unwrap()[1..4].to_owned(), words.next().unwrap()[0..3].to_owned()])
    }).collect();

    let mut steps_num = 0;
    let mut location = &"AAA".to_string();
    while location != "ZZZ" {
        for step in steps {
            match step {
                b'L' => location = &graph[location][0],
                b'R' => location = &graph[location][1],
                _ => panic!("???")
            }
            steps_num += 1;
            if location == "ZZZ" {
                break;
            }
        }
    }
    
    steps_num
}

fn convert(word: &[u8]) -> [u8;3]{
    [word[0], word[1], word[2]]
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let steps = lines.next().unwrap().as_bytes();
    lines.next();
    let graph: HashMap<[u8; 3], [[u8;3]; 2]> = lines.map(|line| {
        let mut words = line.split_whitespace();
        (convert(words.next().unwrap().as_bytes()), [convert(&words.nth(1).unwrap().as_bytes()[1..4]), convert(&words.next().unwrap().as_bytes()[0..3])])
    }).collect();

    let mut starting_pos: Vec<[u8;3]> = input.lines().skip(2).filter_map(|s| {
        let first_word = s.split_whitespace().next().unwrap();
        if first_word.as_bytes()[2] == b'A'{
            Some(convert(first_word.to_string().as_bytes()))
        } else {
            None
        }
    }).collect();
    
    let mut cycles: Vec<u64> = Vec::new();
    for position in starting_pos.iter_mut(){
        let mut steps_num: u64 = 0;
        while position[2] != b'Z' {
            for step in steps {
                match step {
                    b'L' => *position = graph[position][0].to_owned(),
                    b'R' => *position = graph[position][1].to_owned(),
                    _ => panic!("???")
                }

                steps_num += 1;
                if position[2] == b'Z' {
                    cycles.push(steps_num);
                    break;
                }
            }
        }
    }
    
    cycles.iter().fold(cycles[0], |acc, &i| {
        let (mut n_acc, mut n_i) = (acc, i);
        while n_acc != n_i {
            if n_acc < n_i {
                n_acc += acc;
            } else {
                n_i += i;
            }
        }
        n_acc
    })
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).unwrap()));
}
