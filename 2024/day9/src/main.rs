use std::{fmt::Debug, iter};

fn main() {
    let contents = include_str!("../input/puzzle");
    let start = std::time::Instant::now();
    
    part_1(contents);
    part_2(contents);

    println!("Elapsed: {:?}", start.elapsed());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Empty,
    File{ id: u16 },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Block {
    block_type: BlockType,
    length: u8,
}

impl Block {
    fn empty(length: u8) -> Self {
        Self {
            block_type: BlockType::Empty,
            length,
        }
    }

    fn file(id: u16, length: u8) -> Self {
        Self {
            block_type: BlockType::File { id },
            length,
        }
    }
}

fn part_2(contents: &str) {
    let mut filesystem = contents
        .char_indices()
        .map(|(i, c)| {
            match i % 2 {
                0 => Block::file(i as u16 / 2, c.to_digit(10).unwrap() as u8),
                1 => Block::empty(c.to_digit(10).unwrap() as u8),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let mut extract_index = filesystem.len() - 1;
    let mut lookup = [0; 9]; 
    
    while extract_index > 0 {
        match filesystem[extract_index] {
            Block { block_type: BlockType::Empty, .. } => {
                extract_index -= 1;
            } 
            Block { block_type: BlockType::File { id }, length } => {
                let insert_index = filesystem[..extract_index]
                    .iter()
                    .enumerate()
                    .skip(*lookup[..length as usize].iter().max().unwrap())
                    .find(|(_, x)| x.block_type == BlockType::Empty && x.length >= length)
                    .map(|(i, _)| i);
                
                match insert_index {
                    Some(insert_index) if filesystem[insert_index].length == length => {
                        lookup[length as usize - 1] = insert_index + 1;
                        filesystem.swap(extract_index, insert_index);
                        extract_index -= 2;
                    }
                    Some(insert_index) => {
                        let remaining = filesystem[insert_index].length - length;
                        lookup[length as usize - 1] = insert_index + 1;
                        filesystem[insert_index] = Block::file(id, length);
                        filesystem[extract_index] = Block::empty(length);
                        filesystem.insert(insert_index + 1, Block::empty(remaining));
                        extract_index -= 1;
                    }
                    None => {
                        lookup[length as usize - 1] = usize::MAX;
                        extract_index -= 1;
                    }
                }
            }
        }
    }
    
    let result = filesystem
        .iter()
        .flat_map(|block| iter::repeat(block.block_type).take(block.length as usize))
        .enumerate()
        .filter_map(|(i, x)| match x {
            BlockType::File { id } => Some(i as u64 * id as u64),
            BlockType::Empty => None,
        })
        .sum::<u64>();
        
    println!("Result: {}", result);
}

fn part_1(contents: &str) {
    let mut filesystem = contents
        .char_indices()
        .flat_map(|(i, c)| {
            let file = match i % 2 {
                0 => BlockType::File { id: i as u16 / 2 },
                1 =>  BlockType::Empty,
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
        .find_map(|(i, &x)| if x != BlockType::Empty { Some(i) } else { None })
        .unwrap();

    while insert_index < extract_index {
        let offset = filesystem[insert_index..extract_index].iter().position(|&x| x == BlockType::Empty).unwrap();
        filesystem[insert_index + offset] = filesystem[extract_index];
        filesystem[extract_index] = BlockType::Empty;
        insert_index += offset + 1;
        while extract_index > 0 && filesystem[extract_index] == BlockType::Empty {
            extract_index -= 1;
        }
    }

    let result: u64 = filesystem
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| match x {
            BlockType::File { id } => Some((i, id)),
            _ => None,
        })
        .map(|(a,b)| a as u64 * b as u64)
        .sum();

    println!("Result: {}", result);
}
