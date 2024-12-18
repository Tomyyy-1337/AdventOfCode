use std::collections::HashSet;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let contents = include_str!("../input/test");

    part_1(contents);
    part_2(contents);
}

fn read_file(contents: &str) -> Vec<(i32, i32, i32, i32)> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(&contents)
        .map(|capture| {
            let x = capture[1].parse::<i32>().unwrap();
            let y = capture[2].parse::<i32>().unwrap();
            let vx = capture[3].parse::<i32>().unwrap();
            let vy = capture[4].parse::<i32>().unwrap();
            (x,y,vx,vy)
        })
        .collect::<Vec<(i32,i32,i32,i32)>>()
}

fn part_2(contents: &str) { 
    let guards = read_file(contents);

    let mut max_score = 0;
    let mut indx = 0;

    for i in 1.. {
        let guard_pos = guards.iter().map(|(x,y,vx,vy)| {
            let x_new = (((x + vx * i) % WIDTH) + WIDTH) % WIDTH;
            let y_new = (((y + vy * i) % HEIGHT) + HEIGHT) % HEIGHT;
            (x_new, y_new)
        }).collect::<HashSet<(i32,i32)>>();

        if guards.iter().all(|(x,y,_,_)| guard_pos.contains(&(*x,*y))) {
            break;
        }

        let score = calc_inverse_entropy_score(&guard_pos);
        if score > max_score {
            max_score = score;
            indx = i;
        }
    }

    println!("Part 2 Result: {:?}", indx);
}

fn calc_inverse_entropy_score(guards: &HashSet<(i32, i32)>) -> i32 {
    guards
        .iter()
        .filter(|(x, y)| guards.contains(&(*x, y - 1)) )
        .count() as i32
}

fn part_1(contents: &str) {
    let guards = read_file(contents);

    let depth = 100;

    let result = guards
        .into_iter()
        .map(|(x,y,vx,vy)| {
            let x_new = (((x + vx * depth) % WIDTH) + WIDTH) % WIDTH;
            let y_new = (((y + vy * depth) % HEIGHT) + HEIGHT) % HEIGHT;
            (x_new, y_new)
        })
        .fold((0,0,0,0), |(q0, q1, q2, q3), (x, y)| {
            if x == WIDTH / 2 || y == HEIGHT / 2 {
                (q0, q1, q2, q3)
            } else if x < WIDTH / 2 && y < HEIGHT / 2 {
                (q0 + 1, q1, q2, q3)
            } else if x > WIDTH / 2 && y < HEIGHT / 2 {
                (q0, q1 + 1, q2, q3)
            } else if x < WIDTH / 2 && y > HEIGHT / 2 {
                (q0, q1, q2 + 1, q3)
            } else {
                (q0, q1, q2, q3 + 1)
            }
        });

    println!("Part 1 Result: {:?}", result.0 * result.1 * result.2 * result.3);
}
