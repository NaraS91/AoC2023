use std::{fs, collections::{VecDeque, HashSet}};
#[derive(Debug, Clone, Copy)]
enum Direction {
    N = 0,
    W = 1,
    S = 2,
    E = 3
}

#[derive(Clone, Copy)]
struct Beam{
    pos: (i32, i32),
    dir: Direction,
}

impl Beam {
    fn in_bounds(&self, grid: &Vec<Vec<u8>>) -> bool {
        let (i, j) = self.pos;
        i >= 0 && j >= 0 && (i as usize) < grid.len() && (j as usize) < grid[0].len()
    }

    fn forward(&mut self) {
        let (i, j) = self.pos;
        match self.dir {
            Direction::N => self.pos = (i-1, j),
            Direction::E => self.pos = (i, j+1),
            Direction::S => self.pos = (i+1, j),
            Direction::W => self.pos = (i, j-1)
        }
    }

    fn split_vertical(&self) -> [Beam;2]{
        let (i, j) = self.pos;
        
        [Beam{
            pos: (i-1, j),
            dir: Direction::N
        },
        Beam{
            pos: (i+1, j),
            dir: Direction::S
        }]
    }

    fn split_horizontal(&self) -> [Beam;2]{
        let (i, j) = self.pos;
        
        [Beam{
            pos: (i, j+1),
            dir: Direction::E
        },
        Beam{
            pos: (i, j-1),
            dir: Direction::W
        }]
    }

    fn rotate(&mut self, c: u8){
        match c {
            b'\\' if matches!(self.dir, Direction::N) => self.dir = Direction::W,
            b'\\' if matches!(self.dir, Direction::W) => self.dir = Direction::N,
            b'\\' if matches!(self.dir, Direction::E) => self.dir = Direction::S,
            b'\\' => self.dir = Direction::E,
            b'/' if matches!(self.dir, Direction::N) => self.dir = Direction::E,
            b'/' if matches!(self.dir, Direction::W) => self.dir = Direction::S,
            b'/' if matches!(self.dir, Direction::S) => self.dir = Direction::W,
            b'/' => self.dir = Direction::N,
            _ => panic!("???")
        }
    }
}

fn read_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn part1(input: &str) -> u64 {
    let grid = read_input(input);
    energy_value(Beam{pos:(0,0), dir:Direction::E}, &grid)
}

fn energy_value(beam: Beam, grid: &Vec<Vec<u8>>) -> u64 {
    let mut energized = vec![vec![false;grid[0].len()];grid.len()];
    let mut beams = VecDeque::from([beam]);
    let mut seen: HashSet<((i32, i32), i32)> = HashSet::new();
    while !beams.is_empty(){
        let mut beam = beams.pop_back().unwrap();
        while beam.in_bounds(&grid){
            let (i, j) = beam.pos;
            energized[i as usize][j as usize] = true;
            match grid[i as usize][j as usize] {
                b'.' => beam.forward(),
                b'|' if matches!(beam.dir, Direction::N | Direction::S) => beam.forward(),
                b'|' => {
                    for beam in beam.split_vertical() {
                        if !seen.contains(&(beam.pos, beam.dir as i32)){
                            seen.insert((beam.pos, beam.dir as i32));
                            beams.push_back(beam);
                        }
                    };
                    break;
                }
                b'-' if matches!(beam.dir, Direction::E | Direction::W) => beam.forward(),
                b'-' => {
                    for beam in beam.split_horizontal() {
                        if !seen.contains(&(beam.pos, beam.dir as i32)){
                            seen.insert((beam.pos, beam.dir as i32));
                            beams.push_back(beam);
                        }
                    };
                    break;
                },
                c => {beam.rotate(c); beam.forward()}
            }
        }
    }

    energized.iter().map(|row| row.iter().filter(|b| **b).count() as u64).sum()
}

fn part2(input: &str) -> u64 {
    let grid = read_input(input);
    let mut beams = VecDeque::new();
    for i in 0..grid.len(){
        beams.push_back(Beam{pos: (i as i32, 0), dir: Direction::E});
        beams.push_back(Beam{pos: (i as i32, grid[0].len() as i32 - 1), dir: Direction::W});
    }
    for j in 0..grid[0].len(){
        beams.push_back(Beam{pos: (0, j as i32), dir: Direction::S});
        beams.push_back(Beam{pos: (grid.len() as i32 - 1, j as i32), dir: Direction::N});
    }

    beams.iter().map(|beam| energy_value(*beam, &grid)).reduce(|acc, e| acc.max(e)).unwrap()
}

fn main() {
    let path = "./input.txt";
    println!("{}", part2(&fs::read_to_string(path).unwrap()));
}
