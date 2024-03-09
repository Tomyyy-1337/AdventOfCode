use nalgebra as na;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn from_str(str: &str) -> Vec2 {
        let mut iter = str.split(", ").map(|s|s.replace(" ", ""));
        Vec2 { 
            x: iter.next().unwrap().parse().unwrap(), 
            y: iter.next().unwrap().parse().unwrap() 
        }
    }
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
}

impl Particle {
    fn from_str(str: &str) -> Particle {
        let mut iter = str.split(" @ ");
        Particle { 
            position: Vec2::from_str(iter.next().unwrap()), 
            velocity: Vec2::from_str(iter.next().unwrap())
        }
    }
}

const BOUND: std::ops::Range<f64> = 200000000000000.0..400000000000000.0;
fn main() {
    let path = "input/puzzle.txt";
    let particles: Vec<Particle> = std::fs::read_to_string(path).unwrap()
        .lines()
        .map(Particle::from_str)
        .collect();

    let collisions: usize = particles.par_iter().enumerate().map(|(i, p1)| {
        particles.iter().skip(i+1).filter(|p2| {
            let a: na::Matrix<f64,_,_,_> = na::DMatrix::from_row_slice(2, 2, &[p1.velocity.x as f64, -p2.velocity.x as f64, p1.velocity.y as f64, -p2.velocity.y as f64]);
            let b: na::Matrix<f64,_,_,_> = na::DVector::from_row_slice(&[p2.position.x as f64 - p1.position.x as f64, p2.position.y as f64 - p1.position.y as f64]);
            a.lu().solve(&b).is_some_and(| val | {
                let x = val[0] * p1.velocity.x as f64 + p1.position.x as f64;
                let y = val[0] * p1.velocity.y as f64 + p1.position.y as f64;
                BOUND.contains(&x) && BOUND.contains(&y) && val[0] >= 0.0 && val[1] >= 0.0 
            }) 
        }).count()
    }).sum();

    println!("Collisions: {:?}", collisions)
} 