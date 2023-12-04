use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::{BuildHasher, Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Planet {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Planet {
    fn sum(&self) -> i64 {
        Planet::sum_part(&self.pos) + Planet::sum_part(&self.vel)
    }

    fn sum_part(part: &[i64]) -> i64 {
        part.iter().fold(0, |acc, i| acc + i.abs())
    }

    fn hash_axis(&self, mut hasher: &mut dyn Hasher, axis: usize) {
        self.pos[axis].hash(&mut hasher);
        self.vel[axis].hash(&mut hasher);
    }
}

pub fn a12_1() {
    let mut planets = parse();
    for _ in 0..1000 {
        step(&mut planets);
    }

    let energy = planets.iter().fold(0, |acc, planet| acc + (planet.sum()));
    println!("{}", energy);
}

pub fn a12_2() {
    let mut prev_hashes = HashSet::<u64>::new();
    let mut axes = [0u64; 3];

    for (i, steps) in axes.iter_mut().enumerate() {
        let mut planets = parse();
        prev_hashes.clear();
        loop {
            let mut hasher = prev_hashes.hasher().build_hasher();
            planets
                .iter()
                .for_each(|p| p.hash_axis(&mut hasher, i as usize));
            if !prev_hashes.insert(hasher.finish()) {
                println!("axis {}x: {}", i, steps);
                break;
            }
            step(&mut planets);
            *steps += 1;
        }
    }

    println!("total: {}x", lcm3(axes[0], axes[1], axes[2]));
}

fn parse() -> [Planet; 4] {
    let vec = read_to_string("inputs/input-12")
        .expect("Failed to read planets")
        .lines()
        .map(|line| {
            let mut split = line.split('=');
            split.next();
            let x = split
                .next()
                .unwrap()
                .split(',')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let y = split
                .next()
                .unwrap()
                .split(',')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let z = split
                .next()
                .unwrap()
                .split('>')
                .next()
                .unwrap()
                .parse()
                .unwrap();

            Planet {
                pos: [x, y, z],
                vel: [0; 3],
            }
        })
        .collect::<Vec<_>>();
    [vec[0], vec[1], vec[2], vec[3]]
}

fn step(planets: &mut [Planet; 4]) {
    let len = planets.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let mut planet1 = planets[i];
            let mut planet2 = planets[j];
            for i in 0..3 {
                velocity(
                    planet1.pos[i],
                    planet2.pos[i],
                    &mut planet1.vel[i],
                    &mut planet2.vel[i],
                );
            }
            planets[i] = planet1;
            planets[j] = planet2;
        }
    }

    for planet in planets.iter_mut() {
        for i in 0..3 {
            planet.pos[i] += planet.vel[i];
        }
    }
}

fn velocity(this: i64, other: i64, this_vel: &mut i64, other_vel: &mut i64) {
    match () {
        _ if this > other => {
            *this_vel -= 1;
            *other_vel += 1;
        }
        _ if this < other => {
            *this_vel += 1;
            *other_vel -= 1;
        }
        _ => (),
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(a, lcm(b, c))
}
