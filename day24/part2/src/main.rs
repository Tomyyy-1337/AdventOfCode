use std::ops::Sub;
use nalgebra::Vector3;
use num::integer::gcd;

#[derive(Debug, Copy, Clone)]
struct Particle {
    position: Vector3<i64>,
    velocity: Vector3<i64>,
}

impl Particle {
    fn from_str(str: &str) -> Particle {
        let mut iter = str.split(" @ ");
        Particle { 
            position: Particle::parse_vec3(iter.next().unwrap()), 
            velocity: Particle::parse_vec3(iter.next().unwrap())
        }
    }

    fn parse_vec3(str: &str) -> Vector3<i64> {
        Vector3::from_iterator(str.split(", ").map(|s| s.parse().unwrap()))
    }
}

impl Sub for &Particle {
    type Output = Particle;

    fn sub(self, rhs: Self) -> Self::Output {
        Particle {
            position: self.position - rhs.position,
            velocity: self.velocity - rhs.velocity,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Plane {
    n: Vector3<i64> ,
    d: i64,
}

impl Plane {
    fn intersection(a: &Plane, b: &Plane, c: &Plane) -> Vector3<i64>  {
        let denominator = a.n.dot(&b.n.cross(&c.n));
        (b.n.cross(&c.n) * a.d + c.n.cross(&a.n) * b.d + a.n.cross(&b.n) * c.d) / denominator
    }

    fn reduce(&self) -> Plane {
        let gcd = gcd(gcd(gcd(self.d, self.n.x), self.n.y), self.n.z);
        Plane {
            n: self.n / gcd,
            d: self.d / gcd,
        }
    }
}

fn solution_velocities(a: &Particle, b: &Particle) -> Plane {
    let delta: Particle = a - b;
    let normal: Vector3<i64> = delta.position.cross(&delta.velocity);
    let sample_v: Vector3<i64> = delta.position + a.velocity;
    let plane_constant = sample_v.dot(&normal);
    Plane {
        n: normal,
        d: plane_constant,
    }.reduce()
}

fn main() {
    let path = "input/puzzle.txt";
    let particles: Vec<Particle> = std::fs::read_to_string(path).unwrap()
        .lines()
        .take(3)
        .map(Particle::from_str)
        .collect();

    let a: Plane = solution_velocities(&particles[0], &particles[1]);
    let b: Plane = solution_velocities(&particles[0], &particles[2]);
    let c: Plane = solution_velocities(&particles[1], &particles[2]);

    let solution_velocity_particle = Particle {
        position: Vector3::zeros(),
        velocity: Plane::intersection(&a, &b, &c),
    };

    let particle_a: Particle = &particles[0] - &solution_velocity_particle;
    let particle_b: Particle = &particles[1] - &solution_velocity_particle;
    let delta_ba: Vector3<i64> = particle_b.position - particle_a.position;

    let t0: i64 = delta_ba.cross(&particle_b.velocity).x / particle_a.velocity.cross(&particle_b.velocity).x;
    let solution_pos: Vector3<i64> = particle_a.position + particle_a.velocity * t0;
    let pos_sum = solution_pos.x + solution_pos.y + solution_pos.z;

    println!("Summe: {}", pos_sum);
} 