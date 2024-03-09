use std::collections::{VecDeque, HashSet, HashMap};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone)]
struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from_str(str: &str) -> Graph {
        let mut nodes: HashMap<String,Vec<String>> = str.lines().map(|s|{
            let mut iter = s.split(": ");
            let key = iter.next().unwrap();
            let values: Vec<String> = iter.next().unwrap().split_ascii_whitespace().map(|s|s.to_string()).collect();
            (key.to_string(), values)
        }).collect();
        for (k,v) in nodes.clone().into_iter() {
            for n in v {
                nodes.entry(n).or_insert(Vec::new()).push(k.to_string());
            }
        }
        Graph { nodes }
    }

    fn connected_nodes(&self, start: &str) -> usize {
        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::from([start]);
        while let Some(current) = queue.pop_front(){
            for n in self.nodes.get(current).unwrap() {
                if !visited.contains(n.as_str()) {
                    queue.push_back(n);
                    visited.insert(n);
                }
            }
        }
        visited.len()
    }

    fn remove_edge(&mut self, a: &str, b: &str) {
        self.nodes.get_mut(a).unwrap().retain(|x| x != b);
        self.nodes.get_mut(b).unwrap().retain(|x| x != a);
    }

    fn find_shortest_path(&self, start: &str, end: &str) -> Vec<String> {
        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::from([start]);
        let mut parent = HashMap::new();
        while let Some(current) = queue.pop_front() {
            if current == end {
                break;
            }
            for n in self.nodes.get(current).unwrap() {
                if !visited.contains(n.as_str()) {
                    queue.push_back(n);
                    visited.insert(n);
                    parent.insert(n.as_str(), current);
                }
            }
        }
        let mut path: Vec<String> = vec![end.to_string()];
        let mut current = end;
        while current != start {
            current = parent.get(current).unwrap();
            path.push(current.to_string());
        }
        path
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents: String = std::fs::read_to_string(path).unwrap();
    let graph = Graph::from_str(&contents);
    let max_iterations = 100;

    loop {
        let mut graph = graph.clone();
        let keys: Vec<&String> = graph.nodes.keys().collect();
        let edge_counts = (0..max_iterations).into_par_iter().map(|_| {
            let mut rng = thread_rng();
            let random_key_1 = keys.choose(&mut rng).unwrap();
            let random_key_2 = keys.choose(&mut rng).unwrap();
            let path = graph.find_shortest_path(random_key_1, random_key_2);
            path.windows(2).map(|n| {
                let a = n[0].to_string();
                let b = n[1].to_string();
                let key = if a < b { (a, b) } else { (b, a) };
                (key, 1)
            }).collect()
        }).reduce(|| HashMap::new(), |mut acc, v| {
            v.iter().for_each(|(k, v)| *acc.entry(k.clone()).or_insert(0) += v);
            acc
        });
        
        let mut v: Vec<_> = edge_counts.iter().collect();
        v.sort_by_key(|(_,v)| *v);
        let v = v.iter().rev().take(3).map(|(k,_)|k).collect::<Vec<_>>();
        for (a,b) in v.iter() {
            graph.remove_edge(&a, &b);
        }
        let set_a = graph.connected_nodes(&v[0].0);
        let total = set_a * (graph.nodes.len() - set_a);
        if total > 0 {
            println!("Total: {}", total);
            break;
        }
    }
}