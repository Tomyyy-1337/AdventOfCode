use std::fmt::Debug;

fn main() {
    let contents = include_str!("../input/puzzle");
    
    part_1(contents);
    
    let start = std::time::Instant::now();
    part_2(contents);
    println!("Time: {:?}", start.elapsed());
}

#[derive(Debug, PartialEq, Eq)]
struct Block {
    block_type: FileType,
    length: u8,
}

fn part_2(contents: &str) {
    let mut filesystem = contents
    .char_indices()
        .map(|(i, c)| {
            match i % 2 {
                0 => Block {
                    block_type: FileType::File { id: i as u16 / 2 },
                    length: c.to_digit(10).unwrap() as u8,
                }, 
                1 => Block {
                    block_type: FileType::Empty,
                    length: c.to_digit(10).unwrap() as u8,
                },
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
    
    let mut extract_index = filesystem
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, x)| if x.block_type != FileType::Empty { Some(i) } else { None })
        .unwrap();

    while extract_index > 0 {
        match filesystem[extract_index] {
            Block { block_type: FileType::Empty, .. } => {
                extract_index -= 1;
            } 
            Block { block_type: FileType::File { id }, length } => {
                let insert_index = filesystem[0..extract_index]
                    .iter()
                    .position(|x| x.block_type == FileType::Empty && x.length >= length);
                
                match insert_index {
                    Some(insert_index) if filesystem[insert_index].length == length => {
                        filesystem[insert_index] = Block {
                            block_type: FileType::File { id },
                            length,
                        };
                        filesystem[extract_index] = Block {
                            block_type: FileType::Empty,
                            length,
                        };
                        extract_index -= 2;
                    }
                    Some(insert_index) => {
                        let remaining = filesystem[insert_index].length - length;
                        filesystem[insert_index] = Block {
                            block_type: FileType::File { id },
                            length,
                        };
                        filesystem[extract_index] = Block {
                            block_type: FileType::Empty,
                            length,
                        };
                        filesystem.insert(insert_index + 1, Block {
                            block_type: FileType::Empty,
                            length: remaining,
                        });
                        extract_index -= 1;
                    }
                    None => {
                        extract_index -= 1;
                    }
                }
            }
        }
    }
    
    let result = filesystem
        .iter()
        .flat_map(|block| 
            std::iter::repeat(block.block_type).take(block.length as usize)
        )
        .enumerate()
        .filter_map(|(i, x)| match x {
            FileType::File { id } => Some((i, id)),
            _ => None,
        })
        .map(|(a,b)| a as u64 * b as u64)
        .sum::<u64>();
        
        

    println!("Result: {}", result);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileType {
    Empty,
    File{ id: u16 },
}

fn part_1(contents: &str) {
    let mut filesystem = contents
        .char_indices()
        .flat_map(|(i, c)| {
            let file = match i % 2 {
                0 => FileType::File { id: i as u16 / 2 },
                1 =>  FileType::Empty,
                _ => unreachable!(),
            };
            std::iter::repeat(file).take(c.to_digit(10).unwrap() as usize)
        })
        .collect::<Vec<_>>();

    let mut insert_index = 0;
    let mut extract_index = filesystem
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, &x)| if x != FileType::Empty { Some(i) } else { None })
        .unwrap();

    while insert_index < extract_index {
        let offset = filesystem[insert_index..extract_index].iter().position(|&x| x == FileType::Empty).unwrap();
        filesystem[insert_index + offset] = filesystem[extract_index];
        filesystem[extract_index] = FileType::Empty;
        insert_index += offset + 1;
        while extract_index > 0 && filesystem[extract_index] == FileType::Empty {
            extract_index -= 1;
        }
    }

    let result: u64 = filesystem
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| match x {
            FileType::File { id } => Some((i, id)),
            _ => None,
        })
        .map(|(a,b)| a as u64 * b as u64)
        .sum();

    println!("Result: {}", result);
}

