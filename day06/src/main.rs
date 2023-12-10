use std::fs;

fn part1(input: &str) -> u64 {
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut lines = input.lines();
    lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().for_each(|num| {
        times.push(num.parse().unwrap());
    });
    lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().for_each(|num| {
        distances.push(num.parse().unwrap());
    });

    let mut result: u64 = 1;
    for (time, distance) in times.iter().zip(distances){
        let delta = *time * *time - 4 * distance;
        let x1 = (((*time as f64) - f64::sqrt(delta as f64)) / 2. + 0.00001).ceil() as u64;
        let x2 =  (((*time as f64) + f64::sqrt(delta as f64)) / 2. - 0.00001).floor() as u64;

        result = result * (x2 - x1 + 1);

    }
    result
}

fn part2(input: &str) -> u64 {
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut lines = input.lines();
    lines.next().unwrap().replace(" ", "").split(":").nth(1).unwrap().split_whitespace().for_each(|num| {
        times.push(num.parse().unwrap());
    });
    lines.next().unwrap().replace(" ", "").split(":").nth(1).unwrap().split_whitespace().for_each(|num| {
        distances.push(num.parse().unwrap());
    });

    let mut result: u64 = 1;
    for (time, distance) in times.iter().zip(distances){
        let delta = *time * *time - 4 * distance;
        let x1 = (((*time as f64) - f64::sqrt(delta as f64)) / 2. + 0.00001).ceil() as u64;
        let x2 =  (((*time as f64) + f64::sqrt(delta as f64)) / 2. - 0.00001).floor() as u64;

        result = result * (x2 - x1 + 1);

    }
    result
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).unwrap()));
}
