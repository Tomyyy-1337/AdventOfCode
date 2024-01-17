use hashbrown::HashMap;

#[derive(Clone, Copy)]
enum Category {
    X, M, A, S
}

impl Category {
    fn from_char(c: char) -> Category {
        match c {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("Invalid category"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tag  {
    Accept,
    Reject,
    Named(String),
}

impl Tag {
    fn from_str(s: &str) -> Tag {
        match s {
            "A" => Tag::Accept,
            "R" => Tag::Reject,
            _ => Tag::Named(s.to_string()),
        }
    }
}

enum Operator {
    Const(Tag),
    LessThan(Category, i32, Tag),
    GreaterThan(Category, i32, Tag),
}

impl Operator {
    fn from_str(s: &str) -> Operator {
        if s.split(':').count() == 1 {
            return Operator::Const(Tag::from_str(s));
        }
        let mut input = s.split(':');
        let mut operator_chars = input.next().unwrap().chars().into_iter();
        let tag = Tag::from_str(input.next().unwrap());
        let category = Category::from_char(operator_chars.next().unwrap());
        return match operator_chars.next() {
            Some('<') => Operator::LessThan(category, operator_chars.collect::<String>().parse::<i32>().unwrap(), tag),
            Some('>') => Operator::GreaterThan(category, operator_chars.collect::<String>().parse::<i32>().unwrap(), tag),
            _ => panic!("Invalid operator"),
        }
    }

    fn evaluate_range(&self, range: &Range) -> Option<(Tag, Range, Range)> {
        match self {
            Operator::Const(tag) => Some((tag.clone(), range.clone(), Range{ ranges: vec![(0, 0), (0, 0), (0, 0), (0, 0)] })),
            Operator::LessThan (category, value, tag) if range.get_value(category).0 < *value => {
                let range_hit = range.set_value(category, (range.get_value(category).0, *value));
                let range_miss = range.set_value(category, (*value, range.get_value(category).1));
                Some((tag.clone(), range_hit, range_miss))
            },
            Operator::GreaterThan(category, value, tag) if range.get_value(category).1 > *value => {
                let range_hit = range.set_value(category, (*value + 1, range.get_value(category).1));
                let range_miss = range.set_value(category, (range.get_value(category).0, *value + 1));
                Some((tag.clone(), range_hit, range_miss))
            },
            _ => None ,
        }
    }
}

struct Workflow {
    filter: Vec<Operator>,
}

impl Workflow {
    fn from_str(s: &str) -> (Tag, Workflow) {
        let mut s_iter = s.split('{');
        let tag = Tag::from_str(s_iter.next().unwrap());
        let filter = s_iter.next().unwrap().replace("}", "")
            .split(',')
            .map(|s1| Operator::from_str(s1))
            .collect::<Vec<Operator>>();
        (tag, Workflow { filter })
    }
}

#[derive(Clone)]
struct Range {
    ranges: Vec<(i32, i32)>,
}

impl Range { 
    fn get_value(&self, category: &Category) -> (i32, i32) {
        self.ranges[*category as usize]
    }

    fn set_value(&self, category: &Category, value: (i32, i32)) -> Self {
        let mut result = self.clone();
        result.ranges[*category as usize] = value;
        result
    }

    fn abs(&self) -> u64 {
        self.ranges.iter().map(|(a, b)| (b - a) as u64).product()
    }
}
    

fn main() {
    let path = "input/puzzle.txt";
    
    let contents = std::fs::read_to_string(path).unwrap();
    let input = contents.split("\r\n\r\n").next().unwrap();

    let workflow_map: HashMap<Tag, Workflow> = input
        .lines()
        .map(Workflow::from_str)
        .collect();

    let mut ranges = Vec::new();
    let mut stack = vec![(Tag::Named("in".to_string()), Range{ ranges: vec![(1, 4001), (1, 4001), (1, 4001), (1, 4001)] })];
    while let Some((tag, mut range)) = stack.pop() {
        let workflow = workflow_map.get(&tag).unwrap();
        for operator in workflow.filter.iter() {
            if let Some((tag, range_hit, range_miss)) = operator.evaluate_range(&range) {
                match tag {
                    Tag::Named(_) => stack.push((tag, range_hit)),
                    Tag::Accept => ranges.push(range_hit),
                    Tag::Reject => (),
                }
                range = range_miss;
            } 
        }
    }
    let sum = ranges.iter().map(|r| r.abs()).sum::<u64>();
    
    println!("Sum: {}", sum);
}