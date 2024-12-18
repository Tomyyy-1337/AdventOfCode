enum Edge {
    Horizontal {
        y: isize,
        x_start: isize,
        x_end: isize,
    },
    Vertical {
        x: isize,
        y_start: isize,
        y_end: isize,
    }
}

impl Edge {
    fn new_horizontal(y: isize, x_start: isize, x_end: isize) -> Self {
        Edge::Horizontal { y, x_start, x_end }
    }
    fn new_vertical(x: isize, y_start: isize, y_end: isize) -> Self {
        Edge::Vertical { x, y_start, y_end }
    }
}

impl Edge {
    fn combine(&self, other: &Edge) -> Option<Edge> {
        match (self, other) {
            (Edge::Horizontal { y: y1, x_start: x1, x_end: x2 }, Edge::Horizontal { y: y2, x_start: x3, x_end: x4 }) 
                if y1 == y2 && *x2 == x3 - 1 
                    => Some(Edge::Horizontal { y: *y1, x_start: *x1, x_end: *x4 }),
            (Edge::Horizontal { y: y1, x_start: x1, x_end: x2 }, Edge::Horizontal { y: y2, x_start: x3, x_end: x4 }) 
                if y1 == y2 && *x1 == x4 + 1 
                    => Some(Edge::Horizontal { y: *y1, x_start: *x3, x_end: *x2 }),
            (Edge::Vertical { x: x1, y_start: y1, y_end: y2 }, Edge::Vertical { x: x2, y_start: y3, y_end: y4 }) 
                if x1 == x2 && *y2 == y3 - 1 
                    => Some(Edge::Vertical { x: *x1, y_start: *y1, y_end: *y4 }),
            (Edge::Vertical { x: x1, y_start: y1, y_end: y2 }, Edge::Vertical { x: x2, y_start: y3, y_end: y4 }) 
                if x1 == x2 && *y1 == y4 + 1 
                    => Some(Edge::Vertical { x: *x1, y_start: *y3, y_end: *y2 }),
            _ => None,
        }
    }
}

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
        let mut edges = vec![];

        for (indx, &current_char) in self.grid.iter().enumerate() {
            let mut edges_tmp = Vec::new();
            let mut area = 0;
            let mut stack = vec![(indx % self.width, indx / self.width)];
            
            while let Some((x,y)) = stack.pop() {   
                if seen[y * self.width + x] {
                    continue;
                }
                seen[y * self.width + x] = true;
                area += 1;

                Self::OFFSETS.iter()
                    .map(|(dx, dy)| (x as isize + dx, y as isize + dy, dx, dy))
                    .for_each(|(x, y, dx,dy)| match self.get(x, y) {
                        Some(c) if c == current_char => stack.push((x as usize, y as usize)),
                        Some(_) | None => {
                            match (dx, dy) {
                                (0, 1) | ( 0,-1) => edges_tmp.push(Edge::new_horizontal(y * 2 + (*dy == 1) as isize, x, x)),
                                (1, 0) | (-1, 0) => edges_tmp.push(Edge::new_vertical(x * 2 + (*dx == 1) as isize, y, y)),
                                _ => unreachable!(),
                            }
                        },
                    });
            }
            if !edges_tmp.is_empty() {
                edges.push((edges_tmp, area));
            }
        }

        let mut sum = 0;
        for (edges_arr, area) in edges.iter_mut() {
            for i in 0..edges_arr.len() {
                let mut j = i + 1;
                while j < edges_arr.len() {
                    match edges_arr[i].combine(&edges_arr[j]) {
                        Some(combined) => {
                            edges_arr[i] = combined;
                            edges_arr.remove(j);
                            j = i + 1;
                        },
                        None => j += 1,
                    } 
                }
            }
            sum += *area * edges_arr.len();
        }

        println!("Part 2 Result: {}", sum);
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            Some(self.grid[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}
