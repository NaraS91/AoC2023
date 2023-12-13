use std::fs;

fn solution(input: &str, is_reversed: bool) -> i64 {
    input.lines().map(|line| {
        let mut curr_l: Vec<i64> = if is_reversed{
            line.split_whitespace().rev().map(|w| w.parse().unwrap()).collect()
        } else {
            line.split_whitespace().map(|w| w.parse().unwrap()).collect()
        };
        let mut all_zeros = false;
        let mut result: i64 = *curr_l.last().unwrap();
        while !all_zeros {
            all_zeros = true;
            let mut new_l = Vec::new();
            for elems in curr_l.windows(2) {
                new_l.push(elems[1] - elems[0]);
                all_zeros = all_zeros && (elems[1] - elems[0] == 0);
            }
            result += *new_l.last().unwrap();
            curr_l = new_l;
        }

        result
    }).sum()
}

fn _part1(input: &str) -> i64 {
    solution(input, false)
}

fn part2(input: &str) -> i64 {
    solution(input, true)
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).unwrap()));
}
