use std::{collections::HashSet, fs};

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    input.lines().for_each(|line|{
        let mut cards = line.split(':')
            .nth(1)
            .expect("all lines should include cards after :")
            .split("|");
        let card = cards.next().expect("all lines should have a card");
        let winning_numbers =  cards.next().expect("all lines should have winning numbers");
        let winning_numbers: HashSet<u32> = winning_numbers.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut winning_count = 0;
        card.split_whitespace()
            .map(|s| s.parse().unwrap())
            .for_each(|number: u32| {
                if winning_numbers.contains(&number){
                    winning_count += 1;
                }
            });
        if winning_count > 0 {
            sum += u32::pow(2, winning_count - 1);
        }
    });
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut copies: Vec<(u32, u32)> = Vec::new();
    input.lines().for_each(|line|{
        let mut cards = line.split(':')
            .nth(1)
            .expect("all lines should include cards after :")
            .split("|");
        let card = cards.next().expect("all lines should have a card");
        let winning_numbers =  cards.next().expect("all lines should have winning numbers");
        let winning_numbers: HashSet<u32> = winning_numbers.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut winning_count: u32 = 0;
        card.split_whitespace()
            .map(|s| s.parse().unwrap())
            .for_each(|number: u32| {
                if winning_numbers.contains(&number){
                    winning_count += 1;
                }
            });
        
        let current_cards = copies.iter().fold(0, |acc, (_, amount)| acc + *amount) + 1;
        copies = copies.iter_mut().filter_map(|(dur, amount)| {if *dur > 1 {Some((*dur - 1, *amount))} else {None}}).collect();
        if winning_count > 0 {
            copies.push((winning_count, current_cards));
        }
        sum += current_cards;
        
    });
    sum
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).expect("requires input.txt file")));
}
