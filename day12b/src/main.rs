use std::collections::HashMap;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
            _ => panic!("Invalid status Status"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Row {
    springs: Vec<Status>,
    groups: Vec<i32>,
}

impl Row {
    fn from_str(s: &str) -> Row {
        let mut s_iter = s.split_ascii_whitespace();
        let mut status: Vec<Status> = s_iter.next().unwrap().chars().map(Status::from_status).collect();
        status.push(Status::Unknown);
        let mut status_extendet: Vec<Status> = (0..5).into_iter().flat_map(|_| status.clone()).rev().collect();
        status_extendet.remove(0);
        let groups: Vec<i32> = s_iter.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let groups_extendet: Vec<i32> = (0..5).into_iter().flat_map(|_| groups.clone()).rev().collect();
        Row { springs: status_extendet, groups: groups_extendet }
    }

    pub fn count_arrangements(self) -> u64{
        let mut lookup: HashMap<(Vec<Status>, Vec<i32>), u64> = HashMap::new();
        Self::count_rec(&mut lookup, &self.springs, &self.groups)
    }

    fn count_rec(memo: &mut HashMap<(Vec<Status>, Vec<i32>), u64>, springs: &Vec<Status>, groups: &Vec<i32>) -> u64 {
        let key = (springs.clone(), groups.clone());
    
        if let Some(&result) = memo.get(&key) {
            return result;
        }
    
        let result = match groups.as_slice() {
            [] => if !springs.contains(&Status::Damaged) { 1 } else { 0 } 
            [s, ss @ ..] => match springs.as_slice() {
                [] => 0,
                [Status::Operational, cs @ ..] => Self::count_rec(memo, &cs.to_vec(), groups),
                [Status::Damaged, cs @ ..] if cs.len() >= (*s as usize - 1) => {
                    let (sub, rest) = cs.split_at(*s as usize - 1);
                    if sub.contains(&Status::Operational) {
                        return 0;
                    }
                    match rest {
                        [] => if ss.is_empty() { 1 } else { 0 },
                        [Status::Operational | Status::Unknown, rs @ ..] => Self::count_rec(memo, &rs.to_vec(), &ss.to_vec()),
                        _ => 0,
                    }
                }
                [Status::Unknown, cs @ ..] => {
                    let damaged_count = Self::count_rec(memo, &vec![Status::Damaged].into_iter().chain(cs.iter().cloned()).collect(), groups);
                    let operational_count = Self::count_rec(memo, &cs.to_vec(), groups);
                    damaged_count + operational_count
                }
                _ => 0,
            },
        };
        memo.insert(key, result);
        result
    }
}

    
fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(18).build_global().unwrap();

    let path = "input/puzzle.txt";
    let rows: Vec<Row>  = std::fs::read_to_string(path).unwrap().lines().map(Row::from_str).collect();
    
    let sum: u64 = rows.into_par_iter()
        .map(Row::count_arrangements) 
        .sum();

    println!("Sum: {}", sum);
}