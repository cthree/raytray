//! Use Tuple and its operators to calculate the path of a Projectile and
//! plot its position at each `tick` until it hits the ground (the Y
//! coordinate is less than zero). Output the path plot as a PPM format
//! image file `fodder_plot.ppm`.
//!
use raytray::tuple::{Tuple, Coordinate, Vector};
use raytray::canvas::{Canvas, Pixel};
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct World {
    gravity: Tuple,
    wind: Tuple,
}

fn main() -> std::io::Result<()> {
    let env = World {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut ball = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let mut canvas = Canvas::new(10, 10);

    loop {
        ball = tick(ball.clone(), &env);
        canvas.set_pixel(Pixel::from(ball.position.clone()), Tuple::color(1.0, 1.0, 1.0));
        if ball.position.y() <= 0.0 {
            write_ppm_file(&canvas)?;
            break;
        }
    }

    Ok(())
}

fn tick(proj: Projectile, env: &World) -> Projectile {
    Projectile {
        position: proj.position.clone() + proj.velocity.clone(),
        velocity: proj.velocity + env.gravity.clone() + env.wind.clone(),
    }
}

fn write_ppm_file(canvas: &Canvas) -> std::io::Result<()> {
    use raytray::ppm::Ppm;
    use std::fs::File;

    let ppm = format!("{}", Ppm::from(canvas));
    File::create("fodder_plot.ppm")?.write_all(ppm.as_bytes())
}