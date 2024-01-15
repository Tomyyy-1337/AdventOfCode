#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

struct Box {
    id: u32,
    lenses: Vec<Lens>,
}

impl Box {
    fn from_id(id: u32) -> Box {
        Box {
            id,
            lenses: Vec::new(),
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        if let Some(indx) = self.lenses.iter().position(|l| l.label == lens.label) {
            self.lenses[indx] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn remove_lense(&mut self, label: &str) {
        self.lenses = self.lenses.clone().into_iter().filter(|l| l.label != label).collect()
    }

    fn calculate_focus_power(&self) -> u32 {
        self.lenses.iter().enumerate().fold(0, |power, (i,lens)| {
            power + lens.focal_length * (i as u32 + 1)
        }) * (self.id + 1)
    }
}

fn main() {
    let path = "input/puzzle.txt";

    let mut boxes: Vec<Box> = (0..256).into_iter().map(Box::from_id).collect();

    for str in std::fs::read_to_string(path).unwrap().split(',') {
        let label = str.chars().filter(|c| ('A'..='z').contains(c)).collect::<String>();
        let hash = hash(&label);
        if let Some('-') = str.chars().last() {
            boxes[hash as usize].remove_lense(&label);
        } else {
            let focal_length = str.chars().last().unwrap().to_digit(10).unwrap();
            boxes[hash as usize].add_lens(Lens { label, focal_length });
        }
    }

    let sum: u32 = boxes.iter().map(Box::calculate_focus_power).sum();
    println!("Sum: {}", sum);
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |mut acc, c| {
        acc += c as u32;
        acc *= 17;
        acc % 256
    })
}