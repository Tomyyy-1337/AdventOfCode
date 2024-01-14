use std::collections::HashMap;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Operational,
    Unknown,
    Damaged,
}

impl Status {
    fn from_status(c: char) -> Status {
        match c {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!("Invalid Status"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Row {
    springs: Vec<Status>,
    groups: Vec<i32>,
    lookup: HashMap<(Vec<Status>, Vec<i32>), u64>,
}

impl Row {
    fn from_str(s: &str) -> Row {
        let mut s_iter = s.split_ascii_whitespace();
        let mut springs: Vec<Status> = s_iter.next().unwrap().chars().map(Status::from_status).rev().collect();
        springs.push(Status::Unknown);
        springs = (0..5).into_iter().flat_map(|_| springs.clone()).rev().skip(1).collect();
        let mut  groups: Vec<i32> = s_iter.next().unwrap().split(',').filter_map(|x| x.parse::<i32>().ok()).collect();
        groups = (0..5).into_iter().flat_map(|_| groups.clone()).collect();
        Row { springs, groups, lookup: HashMap::new() }
    }

    pub fn count_arrangements(mut self) -> u64 {
        self.count_rec( self.springs.clone(), self.groups.clone())
    }

    fn count_rec(&mut self, springs: Vec<Status>, groups: Vec<i32>) -> u64 {
        let key = (springs.clone(), groups.clone());
        if let Some(&result) = self.lookup.get(&key) {
            return result;
        }
    
        let result = match groups.as_slice() {
            [] => if !springs.contains(&Status::Damaged) { 1 } else { 0 } 
            [s, ss @ ..] => match springs.as_slice() {
                [] => 0,
                [Status::Operational, cs @ ..] => self.count_rec(cs.to_vec(), groups),
                [Status::Damaged, cs @ ..] if cs.len() >= (*s as usize - 1) => {
                    let (sub, rest) = cs.split_at(*s as usize - 1);
                    if sub.contains(&Status::Operational) {
                        return 0;
                    }
                    match rest {
                        [] => if ss.is_empty() { 1 } else { 0 },
                        [Status::Operational | Status::Unknown, rs @ ..] => self.count_rec(rs.to_vec(), ss.to_vec()),
                        _ => 0,
                    }
                }
                [Status::Unknown, cs @ ..] => {
                    let damaged_count = self.count_rec(vec![Status::Damaged].into_iter().chain(cs.iter().cloned()).collect(), groups.clone());
                    let operational_count = self.count_rec(cs.to_vec(), groups);
                    damaged_count + operational_count
                }
                _ => 0,
            },
        };

        self.lookup.insert(key, result);
        result
    }
}

    
fn main() {
    let path = "input/puzzle.txt";

    let sum: u64 = std::fs::read_to_string(path).unwrap()
        .par_lines()
        .map(Row::from_str)
        .map(Row::count_arrangements) 
        .sum();

    println!("Sum: {}", sum);
}