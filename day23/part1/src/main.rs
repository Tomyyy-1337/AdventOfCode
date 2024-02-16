use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Node {
    reachable: Vec<(i32, u32)>,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<i32, Node>,
    start: i32,
    end: i32,
}

impl Graph {
    fn from_str(str: &str) -> Graph {
        let chars: Vec<_> = str.chars().filter(|&c| c != '\n' && c != '\r').collect();
        let nodes_map: Vec<(i32,Node)> = chars.iter().enumerate().filter_map(|(i,c)| match c {
            '.'| '>' | '<' | '^' | 'v' => Some((i as i32, Node { reachable: vec![] })),
            _ => None,
        }).collect();

        
        let start = nodes_map.first().unwrap().0;
        let end = nodes_map.last().unwrap().0;
        
        let mut nodes_map: HashMap<i32,Node> = HashMap::from_iter(nodes_map);
        let width = str.find('\r').unwrap();
        for(i,c) in chars.iter().enumerate() {
            let i = i as i32;
            match c {
                '>' => {
                    let a = i as i32 + 1;
                    if let Some(node) = nodes_map.get_mut(&i) {
                        node.reachable.push((a, 1));
                    }
                },
                '<' => {
                    let a = i as i32 - 1;
                    if let Some(node) = nodes_map.get_mut(&i) {
                        node.reachable.push((a, 1));
                    }
                },
                '^' => {
                    let a = i as i32 - width as i32;
                    if let Some(node) = nodes_map.get_mut(&i) {
                        node.reachable.push((a, 1));
                    }
                },
                'v' => {
                    let a = i as i32 + width as i32;
                    if let Some(node) = nodes_map.get_mut(&i) {
                        node.reachable.push((a, 1));
                    }
                },  
                '.' => {
                    let dirs = vec![(0,1), (0,-1), (1,0), (-1,0)];
                    let mut reachable = vec![];
                    for (dx,dy) in dirs {
                        let x = i % width as i32 + dx;
                        let y = i / width as i32 + dy;
                        let i = x + y * width as i32;
                        if nodes_map.contains_key(&i) {
                            reachable.push((i, 1));
                        }
                    }
                    if let Some(node) = nodes_map.get_mut(&(i as i32)) {
                        node.reachable = reachable;
                    }
                },
                _ => (),
            }
        }
        Graph {
            nodes: nodes_map,
            start,
            end,
        }
    }

    fn find_longest_path(&self) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0, HashSet::new()));
        let mut max_dist = 0;
        while let Some((node, dist, visited)) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            if node == self.end {
                max_dist = max_dist.max(dist);
                continue;
            }
            for (next_node, next_dist) in &self.nodes[&node].reachable {
                let mut visited = visited.clone();
                visited.insert(node);
                queue.push_back((*next_node, dist + next_dist, visited));
            }
        }
        max_dist
    }
}

fn main() {
    let path = "input/puzzle.txt";
    let contents = std::fs::read_to_string(path).unwrap();
    
    let start_time = std::time::Instant::now(); 
    let graph = Graph::from_str(&contents);
    
    println!("Longest Path: {}", graph.find_longest_path());
    println!("Time: {}ms", start_time.elapsed().as_millis());
}