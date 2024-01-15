#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    fn from_char(c: char) -> Status {
        match c {
            '#' => Status::Operational,
            '.' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!("Invalid status char"),
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    status: Vec<Status>,
    groups: Vec<u64>,
}

impl Row {
    fn from_str(s: &str) -> Row {
        let mut s_iter = s.split_ascii_whitespace();
        let status = s_iter.next().unwrap().chars().map(Status::from_char).collect();
        let groups = s_iter.next().unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        Row { status, groups }
    }

    fn valid(&self) -> bool {
        let mut counts = Vec::new();
        let mut num = 0;
        for status in self.status.iter() {
            match status {
                Status::Operational => num += 1,
                Status::Damaged => {
                    if num > 0 {
                        counts.push(num);
                        num = 0;
                    }
                },
                Status::Unknown => panic!("Invalid status"),
            }
        }
        if num > 0 {
            counts.push(num);
        }
        counts == self.groups
    }
}

struct Permutations {
    number_of_elements: u32,
    indx: u64,
}

impl Iterator for Permutations {
    type Item = Vec<Status>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indx < 2u64.pow(self.number_of_elements) {
            let mut v: Vec<Status> = Vec::new();
            for i in 0..self.number_of_elements {
                v.push(match (self.indx >> i) & 1 {
                    0 => Status::Operational,
                    1 => Status::Damaged,
                    _ => panic!("Invalid bit"),
                });
            }
            self.indx += 1;
            return Some(v);
        }
        None
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let rows: Vec<Row>  = std::fs::read_to_string(path).unwrap().lines().map(Row::from_str).collect();

    let sum = rows.iter()
        .map(|row| {
            let perms = Permutations { number_of_elements: row.status.iter().filter(|&s| s == &Status::Unknown).count() as u32, indx: 0 };
            perms.filter_map(|permutation|{
                let mut new_row = row.clone();
                let mut permutation_iter = permutation.iter();
                for (i, status) in row.status.iter().enumerate() {
                    if status == &Status::Unknown {
                        new_row.status[i] = permutation_iter.next().unwrap().clone();
                    }
                }
                if new_row.valid() {
                    Some(new_row)
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        }).flatten()
        .count();

    println!("Sum: {}", sum);
}
