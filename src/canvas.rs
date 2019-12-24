#![allow(unused_macros, dead_code)]

use crate::ppm::Ppm;
use crate::tuple::{Color, Coordinate, Tuple};

struct Row {
    pixels: Box<Vec<Tuple>>,
}

impl Row {
    pub fn new(width: usize) -> Self {
        let mut pixels: Box<Vec<Tuple>> = Box::new(Vec::with_capacity(width));
        for _ in 0..width {
            pixels.push(Tuple::color(0.0, 0.0, 0.0));
        }
        Self { pixels }
    }

    pub fn set_pixel(&mut self, col: usize, color: Tuple) {
        self.pixels[col] = color;
    }
}

#[derive(PartialEq, Debug)]
pub struct Pixel(pub usize, pub usize);

impl From<Tuple> for Pixel {
    fn from(point: Tuple) -> Self {
        Pixel(point.x().round() as usize, point.y().round() as usize)
    }
}

pub struct Canvas {
    height: usize,
    width: usize,
    rows: Box<Vec<Row>>,
}

impl Canvas {
    pub fn new(height: usize, width: usize) -> Self {
        let rows: Box<Vec<Row>> = {
            let mut empty_rows: Vec<Row> = Vec::with_capacity(height);
            for _ in 0..height {
                empty_rows.push(Row::new(width));
            }
            Box::new(empty_rows)
        };

        Self {
            height,
            width,
            rows,
        }
    }

    pub fn set_pixel(&mut self, position: Pixel, color: Tuple) {
        if let Some(row) = self.rows.get_mut(position.0) {
            row.set_pixel(position.1, color);
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl std::ops::Index<Pixel> for Canvas {
    type Output = Tuple;

    fn index(&self, position: Pixel) -> &Self::Output {
        &self.rows[position.0].pixels[position.1]
    }
}

impl From<&Canvas> for Ppm {
    fn from(canvas: &Canvas) -> Ppm {
        let width = canvas.width();
        let height = canvas.height();
        let mut data = Vec::with_capacity(width * height * 3);
        for row in canvas.rows.iter() {
            for pixel in row.pixels.iter() {
                for color in pixel.as_rgb_bytes().iter() {
                    data.push(color.clone());
                }
            }
        }
        Ppm::new(width, height, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_a_row_of_pixels() {
        let row = Row::new(20);
        assert!(20 == row.pixels.len());
    }

    #[test]
    fn test_created_rows_start_as_all_black_pixels() {
        let black_pixel = Tuple::color(0.0, 0.0, 0.0);
        let row = Row::new(20);
        for index in 0..20 {
            assert!(black_pixel == row.pixels[index]);
        }
    }

    #[test]
    fn test_can_set_a_row_pixel_to_a_color() {
        let mut row = Row::new(20);
        row.set_pixel(14, Tuple::color(1.0, 1.0, 1.0));
        assert!(Tuple::color(1.0, 1.0, 1.0) == row.pixels[14]);
        assert!(Tuple::color(1.0, 1.0, 1.0) != row.pixels[13]);
    }

    #[test]
    fn test_can_create_a_canvas() {
        let canvas = Canvas::new(10, 20);
        assert!(10 == canvas.height);
        assert!(20 == canvas.width);
        assert_eq!(10, canvas.rows.len());
        assert_eq!(20, canvas.rows.first().unwrap().pixels.len());
    }

    #[test]
    fn test_can_set_a_canvas_pixel_to_a_color() {
        let mut canvas = Canvas::new(10, 20);
        canvas.set_pixel(Pixel(4, 14), Tuple::color(1.0, 1.0, 1.0));
        let row = &canvas.rows[4];
        assert_eq!(Tuple::color(1.0, 1.0, 1.0), row.pixels[14]);
        let other_row = &canvas.rows[6];
        assert!(Tuple::color(1.0, 1.0, 1.0) != other_row.pixels[14]);
    }

    #[test]
    fn test_can_index_into_a_canvas_with_a_pixel() {
        let mut canvas = Canvas::new(10, 20);
        canvas.set_pixel(Pixel(4, 14), Tuple::color(1.0, 1.0, 1.0));
        assert_eq!(Tuple::color(1.0, 1.0, 1.0), canvas[Pixel(4, 14)]);
    }

    #[test]
    fn test_can_create_a_pixel_from_a_tuple() {
        assert_eq!(Pixel(4, 14), Pixel::from(Tuple::point(4.0, 14.0, 54.3)));
    }

    #[test]
    fn test_points_round_to_nearest_pixel() {
        assert_eq!(Pixel(14, 4), Pixel::from(Tuple::point(13.5, 4.499, 0.0)));
        assert_eq!(Pixel(4, 14), Pixel::from(Tuple::point(3.5, 14.499, 12.0)));
    }

    #[test]
    fn test_constructs_a_ppm_header() {
        let canvas = Canvas::new(10, 20);
        let ppm = Ppm::from(&canvas);
        assert_eq!(
            format!("P3\n{} {}\n255", canvas.width(), canvas.height()),
            ppm.header()
        );
    }

    fn test_ppm(width: usize, height: usize) -> Ppm {
        let mut canvas = Canvas::new(width, height);
        for step in 0..width {
            canvas.set_pixel(Pixel(step, step), Tuple::color(1.0, 1.0, 1.0));
        }
        let ppm = Ppm::from(&canvas);
        ppm
    }

    #[test]
    fn test_constructs_ppm_pixel_data() {
        let width = 5;
        let height = 5;
        let ppm = test_ppm(width, height);
        let body = ppm.body();
        // The PPM body should have the correct number of values
        assert_eq!(width * height, ppm.len());
        // The first pixel of the first row should we white
        assert_eq!(255, body[0]);
        assert_eq!(255, body[1]);
        assert_eq!(255, body[2]);
        // Sonity check that they aren't all white
        assert_eq!(0, body[3]);
        // The second pixel of the second row should be white
        assert_eq!(255, body[3 * width + 3]);
        assert_eq!(255, body[3 * width + 4]);
        assert_eq!(255, body[3 * width + 5]);
    }

    #[test]
    fn test_ppm_output_includes_header() {
        let width = 5;
        let height = 5;
        let ppm = test_ppm(width, height);
        let out = format!("{}", ppm);
        assert!(out.starts_with("P3\n5 5\n255\n"));
    }

    #[test]
    fn test_ppm_lines_are_shorter_than_71_chars() {
        let width = 5;
        let height = 5;
        let ppm = test_ppm(width, height);
        let out = format!("{}", ppm);
        // build an (empty!) iterator over lines > 70 characters long
        let long_line_iter = out.lines().take_while(|x| x.len() > 70);
        assert_eq!(0, long_line_iter.count());
    }

    #[test]
    fn test_ppm_output_ends_with_newline() {
        let width = 5;
        let height = 5;
        let ppm = test_ppm(width, height);
        let out = format!("{}", ppm);
        assert!(out.ends_with("\n"));
    }
}
