#![allow(non_upper_case_globals)]
fn main() {
    let contents = include_str!("../input/puzzle");

    part1(contents);
    part2(contents);
}

const offsets: [[(i32, i32); 4]; 8] = [
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    [(0, 0), (1, -1), (2, -2), (3, -3)],
    [(0, 0), (0, -1), (0, -2), (0, -3)],
    [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
    [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
];

const xmas: &[u8; 4] = b"XMAS";

fn part1(contents: &str) {
    let chars = contents.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let count: usize = (0..chars.len())
        .map(|y_start| {
            (0..chars[y_start].len())
                .map(|x_start| {
                    offsets
                        .iter()
                        .filter(|offset| {
                            offset.iter().zip(xmas.iter()).all(|((y_offset, x_offset), c)| {
                                let y = y_start as i32 + y_offset;
                                let x = x_start as i32 + x_offset;
                                y >= 0 && y < chars.len() as i32 && x >= 0 && x < chars[y as usize].len() as i32 && chars[y as usize][x as usize] == *c
                            })
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum();
    println!("Part_1 Result: {}", count);
}

const search_offsets: [(i32, i32, i32, i32); 4] = [(-1, -1, 1, 1), (-1, 1, 1, -1), (1, -1, -1, 1), (1, 1, -1, -1)];

fn part2(contents: &str) {
    let line_length = contents.lines().next().unwrap().as_bytes().len() as i32 + 2;
    let contents_arr = contents.as_bytes();

    let mut result = 0;

    for y_start in 1..contents.lines().count() - 1 {
        for x_start in 1..line_length - 3 {
            match contents_arr[y_start * line_length as usize + x_start as usize] {
                b'A' if search_offsets
                    .iter()
                    .filter(|(y_offset_1, x_offset_1, y_offset_2, x_offset_2)| {
                        contents_arr[((y_start as i32 + y_offset_1) * line_length + x_start + x_offset_1) as usize] == b'M'
                            && contents_arr[((y_start as i32 + y_offset_2) * line_length + x_start + x_offset_2) as usize] == b'S'
                    })
                    .count() == 2 => result += 1,
                _ => {}
            }
        }
    }
    println!("Part_2 Result: {}", result);
}