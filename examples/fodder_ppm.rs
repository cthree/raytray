//! Use Tuple and its operators to calculate the path of a Projectile and
//! plot its position at each `tick` until it hits the ground (the Y
//! coordinate is less than zero). Output the path plot as a PPM format
//! image file `fodder_plot.ppm`.
//!
use raytray::canvas::{Canvas, Pixel};
use raytray::color::Color;
use raytray::units::{Point3D, Vector3D};
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Projectile {
    position: Point3D,
    velocity: Vector3D,
}

struct World {
    gravity: Vector3D,
    wind: Vector3D,
}

fn main() -> std::io::Result<()> {
    let env = World {
        gravity: Vector3D::new(0.0, -0.1, 0.0),
        wind: Vector3D::new(-0.01, 0.0, 0.0),
    };

    let starting_point = Point3D::new(0.0, 1.0, 0.0);
    let starting_velocity = Vector3D::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut ball = Projectile {
        position: starting_point,
        velocity: starting_velocity,
    };

    let mut canvas = Canvas::new(900, 550);

    loop {
        ball = tick(ball, &env);
        //  Save the plot when the ball moves outside the canvas
        if !canvas.in_bounds(ball.position) {
            write_ppm_file(&canvas)?;
            break;
        }
        canvas.set_pixel(Pixel::from(ball.position), Color::rgb(1.0, 0.5, 0.5));
    }

    Ok(())
}

fn tick(proj: Projectile, env: &World) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn write_ppm_file(canvas: &Canvas) -> std::io::Result<()> {
    use raytray::ppm::Ppm;
    use std::fs::File;

    let drawing_canvas = canvas.clone().flip_vertical();
    let ppm = format!("{}", Ppm::from(&drawing_canvas));
    File::create("fodder_plot.ppm")?.write_all(ppm.as_bytes())
}
