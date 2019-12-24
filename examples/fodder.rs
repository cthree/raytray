//! Use Tuple and its operators to calculate the path of a Projectile and
//! output its position at each `tick` until it hits the ground (the Y
//! coordinate is less than zero).
//!
use raytray::tuple::{Tuple, Coordinate, Vector};

#[derive(Debug, Clone)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct World {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let env = World {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut ball = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    println!("BANG!");

    loop {
        ball = tick(ball.clone(), &env);
        if ball.position.y() <= 0.0 {
            println!("BOOM!");
            break;
        }
        println!("{}", ball.position);
    }
}

fn tick(proj: Projectile, env: &World) -> Projectile {
    Projectile {
        position: proj.position.clone() + proj.velocity.clone(),
        velocity: proj.velocity + env.gravity.clone() + env.wind.clone(),
    }
}
