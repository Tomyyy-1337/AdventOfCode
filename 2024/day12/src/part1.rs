pub struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn from_str(content: &str) -> Self {
        Grid {
            grid: content
                .lines()
                .flat_map(|line| line.chars())
                .map(|c| match c {
                    'A'..='Z' => c as u8 - b'A',
                    _ => panic!("Invalid character in input"),
                })
                .collect(),
            width: content.lines().next().unwrap().len(),
            height: content.lines().count(),
        }
    }

    const OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

    pub fn solve(&self) {
        let mut seen = vec![false; self.grid.len()];
        let mut result = 0;

        for indx in 0..self.grid.len() {
            let mut area = 0;
            let mut fence = 0;

            let current_char = self.grid[indx];
            let mut stack = vec![(indx % self.width, indx / self.width)];
            
            while let Some((x,y)) = stack.pop() {   
                if seen[y * self.width + x] {
                    continue;
                }
                seen[y * self.width + x] = true;
                area += 1;

                Self::OFFSETS.iter()
                    .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                    .for_each(|(x, y)| match self.get(x, y) {
                        Some(c) if c == current_char => stack.push((x as usize, y as usize)),
                        Some(_) | None => fence += 1,
                    });
            }
            result += area * fence;
        }
        println!("Part 1 Result: {}", result);
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            Some(self.grid[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

}