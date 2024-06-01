use std::collections::VecDeque;

fn main() {
    let contets = std::fs::read_to_string("input/puzzle.txt").unwrap().lines().filter(|line| !line.starts_with("$ ls")).collect::<Vec<_>>().join("\n");
    let node = Node::from_str(&contets);

    println!("{}", node.to_string(0));

    let sum = node.find_dirs(100000).into_iter().map(|node| match node {
        Node::Directory { size, .. } => size,
        _ => panic!("Not a directory"),
    }).sum::<u32>();

    println!("Summe: {}", sum);

}

enum Node {
    File {
        name: String,
        size: u32,
    },
    Directory {
        name: String,
        size: u32,
        content: Vec<Node>,
    },
}

impl Node {
    fn from_str(s: &str) -> Node {
        let mut current_path = Vec::new();
        let mut node = Node::Directory {
            name: String::from("/"),
            size: 0,
            content: Vec::new(),
        };
        let mut ptr = &mut node;

        for line in s.lines() {
            if let Some(dir) = line.split("$ cd ").nth(1) {
                match dir {
                    ".." => {current_path.pop();},
                    "/" => current_path.clear(),
                    _ => current_path.push(dir),
                }
                ptr = current_path.iter().fold(&mut node, |acc, dir| {
                    match acc {
                        Node::Directory { content, .. } => {
                            content.iter_mut().find(|node| match node {
                                Node::Directory { name, .. } => name == dir,
                                _ => false,
                            }).unwrap()
                        }
                        _ => panic!("Not a directory"),
                    }
                });
            } else if let Some(dir) = line.split("dir ").nth(1) {
                match ptr {
                    Node::Directory { content, .. } => content.push(Node::Directory {
                        name: String::from(dir),
                        size: 0,
                        content: Vec::new(),
                    }),
                    _ => panic!("Not a directory"),
                }  
            } else {
                let mut parts = line.split_whitespace();
                match ptr {
                    Node::Directory { content, .. } => content.push(Node::File {
                        size: parts.next().unwrap().parse().unwrap(),
                        name: String::from(parts.next().unwrap()),
                    }),
                    _ => panic!("Not a directory"),
                }
            }
        }
        node.update_size();
        node
    }

    fn update_size(&mut self) -> u32 {
        match self {
            Node::File { size, .. } => *size,
            Node::Directory { size, content, .. } => {
                *size = content.iter_mut().map(|node| node.update_size()).sum();
                *size
            }
        }
    }

    fn to_string(&self, current_indent: usize) -> String {
        match self {
            Node::File { name, size } => format!("{:width$}{} (file, {})\n", "", name, size, width = current_indent),
            Node::Directory { name, size, content } => {
                let mut s = format!("{:width$}{} (dir, {})\n", "", name, size, width = current_indent);
                for node in content {
                    s.push_str(&node.to_string(current_indent + 4));
                }
                s
            }
        }
    }

    fn find_dirs(&self, size_limit: u32) -> Vec<&Node> {
        let mut dirs = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while let Some(node) = queue.pop_front() {
            match node {
                Node::File { .. } => (),
                Node::Directory { size, content, .. } => {
                    if *size <= size_limit {
                        dirs.push(node);
                    }
                    for node in content {
                        queue.push_back(node);
                    }
                }
            }
        }
        dirs
    }
}