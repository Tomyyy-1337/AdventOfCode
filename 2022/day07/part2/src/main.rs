fn main() {
    let contets = std::fs::read_to_string("input/puzzle.txt").unwrap();
    let node = Node::from_str(&contets);

    println!("{}", node.to_string(0));

    let current_size = match node {
        Node::Directory { size, .. } => size,
        _ => panic!("Not a directory"),
    };

    let needed_space = 30000000 - (70000000 - current_size);
    let sizes = node.dir_sizes();
    let best = sizes.into_iter().fold(u32::MAX, |acc, f| if f >= needed_space && f < acc {f} else {acc});

    println!("{}", best);
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

        for block in s.split("$ ").skip(2)  {
            match block.split_whitespace().next().unwrap()  {
                "cd" => {
                    match block.split("cd ").nth(1).unwrap().trim() {
                        ".." => {current_path.pop();},
                        "/" => current_path.clear(),
                        dir => current_path.push(dir),
                    }
                    ptr = current_path.iter().fold(&mut node, |acc, dir: &&str| match acc {
                        Node::Directory { content, .. } => 
                            content.iter_mut().find(|node| node.is_dir_with_name(dir)).unwrap(),
                        _ => panic!("Not a directory"),
                    });
                }
                "ls" => {
                    block.lines().skip(1).for_each(|line| match line.split_whitespace().next().unwrap() {
                        "dir" => match ptr {
                            Node::Directory { content, .. } => content.push(Node::Directory {
                                name: String::from(line.split("dir ").nth(1).unwrap()),
                                size: 0,
                                content: Vec::new(),
                            }),
                            _ => panic!("Not a directory"),
                        },
                        _ => {
                            let mut parts = line.split_whitespace();
                            match ptr {
                                Node::Directory { content, .. } => content.push(Node::File {
                                    size: parts.next().unwrap().parse().unwrap(),
                                    name: String::from(parts.next().unwrap()),
                                }),
                                _ => panic!("Not a directory"),
                            }
                        }
                    });
                }
                _ => panic!("Unknown command")
            }
        }
        node.update_size();
        node
    }

    fn is_dir_with_name(&self, name: &str) -> bool {
        match self {
            Node::Directory { name: n, .. } => n == name,
            _ => false,
        }
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

    fn dir_sizes(&self) -> Vec<u32> {
        match self {
            Node::File { .. } => Vec::new(),
            Node::Directory { size, content, .. } => {
                let mut sizes = vec![*size];
                for node in content {
                    sizes.extend(node.dir_sizes());
                }
                sizes
            }
        }
    }
}