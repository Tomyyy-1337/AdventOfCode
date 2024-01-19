use std::collections::VecDeque;

use hashbrown::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Module {
    FlipFlop(Pulse, Vec<String>),
    Conjunction(Vec<(String, Pulse)>, Vec<String>),
    Broadcast(Vec<String>),
}

impl Module {
    fn from_str(s: &str) -> (String, Module) {
        let mut iter = s.split(" -> ");
        match iter.next().unwrap() {
            "broadcaster" => ("broadcaster".to_string(), Module::Broadcast(iter.next().unwrap().split(", ").map(|s| s.to_string()).collect())),
            s if s.starts_with('%') => (s[1..].to_string(), Module::FlipFlop(Pulse::Low, iter.next().unwrap().split(", ").map(|s| s.to_string()).collect())),
            s if s.starts_with('&') => (s[1..].to_string(), Module::Conjunction(Vec::new(), iter.next().unwrap().split(", ").map(|s| s.to_string()).collect())),
            _ => panic!("Invalid module type: {}", s),
        }
    }

    fn process_pulse(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::Broadcast(_) => Some(Pulse::Low),
            Module::FlipFlop(mem, _) => {
                if pulse == Pulse::Low {
                    *mem = match mem {
                        Pulse::High => Pulse::Low,
                        Pulse::Low => Pulse::High,
                    };
                    return Some(mem.clone());
                }
                None
            },
            Module::Conjunction(inputs, _) => {
                match inputs.iter().position(|(s, _)| s == from) {
                    Some(i) => inputs[i] = (from.to_string(), pulse),
                    None => panic!("Invalid input: {}", from),
                }
                if inputs.iter().find(|(_,a)| a == &Pulse::Low).is_none() {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            },
        }
    }

    fn get_outputs(&self) -> &Vec<String> {
        match self {
            Module::Broadcast(outputs) => outputs,
            Module::FlipFlop(_, outputs) => outputs,
            Module::Conjunction(_, outputs) => outputs,
        }
    }

}

fn main() {
    let path = "input/puzzle.txt";
    let mut contents: HashMap<String, Module> = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(|s| Module::from_str(s))
        .collect();

    let cc = contents.clone();
    contents.iter_mut().for_each(|(k,v)| {
        if let Module::Conjunction(inputs, _) = v {
            cc.iter().for_each(|(k2,v2)| {
                if v2.get_outputs().contains(k) {
                    inputs.push((k2.to_string(), Pulse::Low));
                }
            });
        }
    });

    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("".to_string(),"broadcaster".to_string(), Pulse::Low));
        low_count += 1;
        while let Some((from,tag, pulse)) = queue.pop_back() {
            if let Some(module) = contents.get_mut(&tag) {
                let outputs = module.get_outputs().clone();
                match module.process_pulse(&from, pulse) {
                    Some(result_pulse) => {
                        if result_pulse == Pulse::High {
                            high_count += outputs.len();
                        } else {
                            low_count += outputs.len();
                        }
                        for output in outputs {
                            queue.push_front((tag.to_string(), output, result_pulse));
                        }
                    },
                    None => (),
                }
            }
        }
    }
        
    println!("High count: {}", high_count);
    println!("Low count: {}", low_count);
    println!("Product: {}", high_count * low_count)

}
