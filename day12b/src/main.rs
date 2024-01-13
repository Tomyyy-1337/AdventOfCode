use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
struct Row {
    status: Vec<Status>,
    groups: Vec<i64>,
    first_group_changed: bool,
}

impl Row {
    fn from_str(s: &str) -> Row {
        let mut s_iter = s.split_ascii_whitespace();
        let mut status: Vec<Status> = s_iter.next().unwrap().chars().map(Status::from_char).collect();
        status.push(Status::Operational);
        let groups: Vec<i64> = s_iter.next().unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        let first_group_changed = false;
        Row { status, groups, first_group_changed }
    }

    fn count_valid(&mut self) -> u64 {
        match self.status.first() {
            Some(Status::Operational) => {
                if self.groups[0] == 0 {
                    self.groups.remove(0);
                    self.first_group_changed = false;
                }
                if self.first_group_changed {
                    return 0;
                }
                if self.groups.len() == 0 {
                    if self.status.contains(&Status::Damaged) {
                        return 0;
                    } else {
                        return 1;
                    }
                }
                self.status.remove(0);
                return self.count_valid();
            },
            Some(Status::Damaged) => {
                self.groups[0] -= 1;
                self.first_group_changed = true;
                if self.groups[0] < 0 {
                    return 0;
                }
                self.status.remove(0);
                return self.count_valid();
            },
            Some(Status::Unknown) => {
                let mut row1 = self.clone();
                let mut row2 = self.clone();
                row1.status[0] = Status::Operational;
                row2.status[0] = Status::Damaged;
                return row1.count_valid() + row2.count_valid();
            },
            None => return 0,
        }
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let mut rows: Vec<Row>  = std::fs::read_to_string(path).unwrap().lines().map(Row::from_str).collect();

    let sum: u64 = rows.par_iter_mut()
        .progress()
        .map(Row::count_valid) 
        .sum();

    println!("Sum: {}", sum);
}