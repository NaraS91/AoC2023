use std::{fs, collections::VecDeque};



fn read_input(input: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>){
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    let lines = &mut input.lines();
    lines.next().unwrap().split_whitespace().for_each(|s| if let Ok(num) = s.parse() {seeds.push(num)} else {});
    while let Some(_) = lines.find(|s| s.ends_with(":")) {
        let mut map: Vec<(u64,u64, u64)> = Vec::new();
        for line in lines.take_while(|s| !s.is_empty()){
            let mut numbers = line.split_whitespace();
            map.push((numbers.next().unwrap().parse().unwrap(),
                numbers.next().unwrap().parse().unwrap(),
                numbers.next().unwrap().parse().unwrap()))
        }
        maps.push(map);
    }
    (seeds, maps)
}

fn _part1(input: &str) -> u64 {
    let (seeds, maps) = read_input(input);
    let mut result = u64::MAX;
    for seed in seeds{
        let mut id = seed;
        for map in maps.iter() {
            for (dest, source, length) in map {
                if id >= *source && id < *source + *length{
                    id = *dest + (id - *source);
                    break;
                }
            }
        }
        result = result.min(id);
    }
    result
}

fn range_maps((start, end): (u64, u64), (dest, src, length): (u64, u64, u64), remaining: &mut VecDeque<(u64, u64)>) -> Option<(u64, u64)> {
    let sub_range_l = start.max(src);
    let sub_range_r = (end).min(src + length);
    if sub_range_l < sub_range_r {
        if start < src {
            remaining.push_back((start, sub_range_l));
        } 
        if end > src + length {
            remaining.push_back((sub_range_r, end));
        }
        Some((dest+(sub_range_l-src), dest+(sub_range_r-src)))
    } else {
        remaining.push_back((start, end));
        None
    }
}

fn part2(input: &str) -> u64 {
    let (seeds, maps) = read_input(input);
    let mut ranges: VecDeque<(u64, u64)> = seeds.windows(2).step_by(2).map(|seed| (seed[0], seed[0] + seed[1])).collect();

    for map in maps {
        let mut next_ranges = VecDeque::new();    
        for map_elem in map {
            ranges = ranges.iter().fold(VecDeque::new(), |mut remaining, range| {
                if let Some(r) = range_maps(*range, map_elem, &mut remaining) {
                    next_ranges.push_back(r);
                }
                remaining
            });
        }
        ranges.append(&mut next_ranges);
    }

    ranges.iter().fold(u64::MAX, |acc, (start, _)| acc.min(*start))
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).unwrap()));
}
