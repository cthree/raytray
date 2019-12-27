use crate::color::Color;
use crate::units::Point3D;
use std::fmt;
use textwrap;

#[derive(Debug, Clone)]
struct Row {
    width: usize,
    pixels: Vec<Color>,
}

impl Row {
    pub fn new(width: usize) -> Self {
        let mut pixels: Vec<Color> = Vec::with_capacity(width);
        for _ in 0..width {
            pixels.push(Color::rgb(0.0, 0.0, 0.0));
        }
        Self { width, pixels }
    }

    pub fn set_pixel(&mut self, col: usize, color: Color) {
        if col >= self.width {
            panic!(
                "attempt to set pixel beyond row boundary {} > {}!",
                col, self.width,
            );
        }

        self.pixels[col] = color;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pixel(usize, usize);

impl Pixel {
    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }
}

impl From<Point3D> for Pixel {
    // Convert a Point3D into a canvas Pixel
    fn from(point: Point3D) -> Self {
        Pixel(point.x().round() as usize, point.y().round() as usize)
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    height: usize,
    width: usize,
    rows: Vec<Row>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rows: Vec<Row> = Vec::with_capacity(height);
        for _ in 0..height {
            rows.push(Row::new(width));
        }

        Self {
            height,
            width,
            rows,
        }
    }

    pub fn set_pixel(&mut self, position: Pixel, color: Color) {
        if position.y() > self.height() - 1 {
            panic!(
                "assempt to set pixel beyond height boundary {} > {}!",
                position.y(),
                self.height() - 1,
            );
        }

        if let Some(row) = self.rows.get_mut(position.y()) {
            row.set_pixel(position.x(), color);
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, position: Point3D) -> bool {
        let pixel: Pixel = position.into();
        pixel.x() < self.width() && pixel.y() < self.height()
    }
}

impl std::ops::Index<Pixel> for Canvas {
    type Output = Color;

    fn index(&self, position: Pixel) -> &Self::Output {
        &self.rows[position.y()].pixels[position.x()]
    }
}

pub struct Ppm {
    width: usize,
    height: usize,
    body: Vec<u8>,
}

impl Ppm {
    pub fn new(width: usize, height: usize, body: Vec<u8>) -> Self {
        Ppm {
            width,
            height,
            body,
        }
    }

    pub fn header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height)
    }

    pub fn body(&self) -> &[u8] {
        self.body.as_slice()
    }

    pub fn len(&self) -> usize {
        self.body.len() / 3
    }

    pub fn is_empty(&self) -> bool {
        self.body.len() == 0
    }
}

impl From<&Canvas> for Ppm {
    fn from(canvas: &Canvas) -> Self {
        let width = canvas.width;
        let height = canvas.height;
        let mut data: Vec<u8> = Vec::with_capacity(width * height * 3);
        for row in canvas.rows.iter().rev() {
            for pixel in row.pixels.iter() {
                for color in pixel.as_rgb_bytes().iter() {
                    data.push(*color);
                }
            }
        }

        Self {
            width,
            height,
            body: data,
        }
    }
}

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s_values: Vec<_> = self.body.iter().map(|c| format!("{}", c)).collect();
        let body = s_values.join(" ");
        write!(f, "{}\n{}\n", self.header(), textwrap::fill(&body, 70))
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
        let black_pixel = Color::rgb(0.0, 0.0, 0.0);
        let row = Row::new(20);
        for index in 0..20 {
            assert!(black_pixel == row.pixels[index]);
        }
    }

    #[test]
    fn test_can_set_a_row_pixel_to_a_color() {
        let mut row = Row::new(20);
        row.set_pixel(14, Color::rgb(1.0, 1.0, 1.0));
        assert!(Color::rgb(1.0, 1.0, 1.0) == row.pixels[14]);
        assert!(Color::rgb(1.0, 1.0, 1.0) != row.pixels[13]);
    }

    #[test]
    fn test_can_create_a_canvas() {
        let canvas = Canvas::new(20, 10);
        assert!(10 == canvas.height);
        assert!(20 == canvas.width);
        assert_eq!(10, canvas.rows.len());
        assert_eq!(20, canvas.rows.first().unwrap().pixels.len());
    }

    #[test]
    fn test_can_set_a_canvas_pixel_to_a_color() {
        let mut canvas = Canvas::new(20, 20);
        canvas.set_pixel(Pixel(4, 14), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(canvas[Pixel(4, 14)], Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(canvas[Pixel(3, 14)], Color::rgb(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_can_index_into_a_canvas_with_a_pixel() {
        let mut canvas = Canvas::new(20, 20);
        canvas.set_pixel(Pixel(4, 14), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(Color::rgb(1.0, 1.0, 1.0), canvas[Pixel(4, 14)]);
    }

    #[test]
    fn test_can_create_a_pixel_from_a_point() {
        assert_eq!(Pixel(4, 14), Pixel::from(Point3D::new(4.0, 14.0, 54.3)));
    }

    #[test]
    fn test_points_round_to_nearest_pixel() {
        assert_eq!(Pixel(14, 4), Pixel::from(Point3D::new(13.5, 4.499, 0.0)));
        assert_eq!(Pixel(4, 14), Pixel::from(Point3D::new(3.5, 14.499, 12.0)));
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
            canvas.set_pixel(Pixel(step, step), Color::rgb(1.0, 1.0, 1.0));
        }
        let ppm = Ppm::from(&canvas);
        ppm
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

    #[test]
    fn test_can_check_if_a_point_is_within_canvas() {
        let canvas = Canvas::new(10, 10);
        assert!(canvas.in_bounds(Point3D::new(9.0, 9.0, 9.0)));
        assert!(!canvas.in_bounds(Point3D::new(9.0, 10.0, 9.0)));
        assert!(!canvas.in_bounds(Point3D::new(10.0, 9.0, 9.0)));
        assert!(!canvas.in_bounds(Point3D::new(10.0, 10.0, 10.0)));
    }
}
