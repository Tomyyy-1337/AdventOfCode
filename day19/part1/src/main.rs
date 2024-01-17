use hashbrown::HashMap;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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

    fn evaluate(&self, part: &Part) -> Option<Tag> {
        match self {
            Operator::Const(tag) => Some(tag.clone()),
            Operator::LessThan   (category, value, tag) if part.get_value(category) < *value => Some(tag.clone()),
            Operator::GreaterThan(category, value, tag) if part.get_value(category) > *value => Some(tag.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Part {
    values: Vec<i32>,
}

impl Part {
    fn from_str(s: &str) -> Part {
        Part { 
            values: s.split(',')
                .map(|s1| s1.chars().filter(|c| c.is_digit(10)).collect::<String>())
                .flat_map(|s2| s2.parse::<i32>())
                .collect(),
        }
    }

    fn get_value(&self, category: &Category) -> i32 {
        self.values[*category as usize]
    }
}

#[derive(Debug)]
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

    fn evaluate(&self, part: &Part) -> Tag {
        for filter in &self.filter {
            if let Some(tag) = filter.evaluate(part) {
                return tag;
            }
        }
        panic!("No Tag found");
    }
}

fn main() {
    let path = "input/puzzle.txt";
    
    let contents = std::fs::read_to_string(path).unwrap();
    let mut input = contents.split("\r\n\r\n");
    
    let workflow_map: HashMap<Tag, Workflow> = input.next().unwrap()
        .lines()
        .map(Workflow::from_str)
        .collect();

    let parts: Vec<Part> = input.next().unwrap()
        .lines()
        .map(Part::from_str)
        .collect();

    let sum: i32 = parts.iter()
        .filter(|part| {
            let mut workflow_tag = Tag::from_str("in");
            while let Tag::Named(_) = &workflow_tag {
                workflow_tag = workflow_map.get(&workflow_tag).unwrap().evaluate(part);
            }
            workflow_tag == Tag::Accept 
        }).map(|part| part.values.iter().sum::<i32>())
        .sum();

    println!("Sum: {}", sum);
}