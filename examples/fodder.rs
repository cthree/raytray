//! Use Tuple and its operators to calculate the path of a Projectile and
//! output its position at each `tick` until it hits the ground (the Y
//! coordinate is less than zero).
//!
use raytray::units::{Point3D, Vector3D};

#[derive(Debug, Clone)]
struct Projectile {
    position: Point3D,
    velocity: Vector3D,
}

struct World {
    gravity: Vector3D,
    wind: Vector3D,
}

fn main() {
    let env = World {
        gravity: Vector3D::new(0.0, -0.1, 0.0),
        wind: Vector3D::new(-0.01, 0.0, 0.0),
    };

    let mut ball = Projectile {
        position: Point3D::new(0.0, 1.0, 0.0),
        velocity: Vector3D::new(1.0, 1.0, 0.0).normalize(),
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
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
