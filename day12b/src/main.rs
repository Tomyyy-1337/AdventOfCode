use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    fn from_char(c: char) -> Status {
        match c {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!("Invalid status char"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Row {
    status: Vec<Status>,
    groups: Vec<i32>,
    first_group_changed: bool,
}

impl Row {
    fn from_str(s: &str) -> Row {
        let mut s_iter = s.split_ascii_whitespace();
        let status: Vec<Status> = s_iter.next().unwrap().chars().map(Status::from_char).collect();
        let mut status_extendet: Vec<Status> = (0..5).into_iter().flat_map(|_| {
            let mut arr = status.clone();
            arr.push(Status::Unknown);
            arr
        }).collect();
        let indx = status_extendet.len() - 1;
        status_extendet[indx] = Status::Operational;
        let groups: Vec<i32> = s_iter.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let groups_extendet: Vec<i32> = (0..5).into_iter().flat_map(|_| groups.clone()).rev().collect();
        let first_group_changed = false;
        Row { status: status_extendet, groups: groups_extendet, first_group_changed }
    }

    fn count_valid(&mut self) -> u64 {
        let mut result = 0;
        let mut stack: Vec<Row> = vec![self.clone()];
        
        while let Some(mut current) = stack.pop() {
            match current.status.first() {
                Some(Status::Operational) => {
                    if current.groups.iter().sum::<i32>() > current.status.iter().filter(|&&s| s != Status::Operational).count() as i32 {
                        continue;
                    }
                    if current.groups.last().unwrap() == &0 {
                        current.groups.pop();
                        current.first_group_changed = false;
                    }
                    if current.first_group_changed {
                        continue;
                    }
                    if current.groups.len() == 0 {
                        if current.status.contains(&Status::Damaged) {
                            continue;
                        } else {
                            result += 1;
                            continue;
                        }
                    }
                    current.status.remove(0);
                    stack.push(current);
                },
                Some(Status::Damaged) => {
                    let indx = current.groups.len() - 1;
                    if current.groups[indx] == 0 {
                        continue;
                    }
                    let operational_indx = (current.status.iter().position(|x| *x == Status::Operational).unwrap() as i32).min(current.groups[indx]);
                    current.groups[indx] -= operational_indx;
                    current.first_group_changed = true;
                    current.status.drain(..operational_indx as usize);
                    stack.push(current);
                },
                Some(Status::Unknown) => {
                    if current.groups.last().unwrap() == &0 {
                        current.status[0] = Status::Operational;
                        stack.push(current);
                        continue;
                    }
                    let mut row1 = current.clone();
                    row1.status[0] = Status::Operational;
                    current.status[0] = Status::Damaged;
                    stack.push(row1);
                    stack.push(current);
                },
                None => continue,
            }
        }
        result
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(18).build_global().unwrap();

    let path = "input/test.txt";
    let mut rows: Vec<Row>  = std::fs::read_to_string(path).unwrap().lines().map(Row::from_str).collect();

    //start timer
    let now = std::time::Instant::now();
    let sum: u64 = rows.par_iter_mut()
        .progress()
        .map(Row::count_valid) 
        .sum();
    //stop timer
    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);
    println!("Sum: {}", sum);
}