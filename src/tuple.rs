#[derive(Debug, PartialEq)]
pub struct Float {
    inner: f32,
}

impl Float {

}

#[derive(PartialEq, Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert!(true);
    }

    #[test]
    fn test_a_vector_w_is_0() {
        assert_eq!(Tuple::vector(0.0,0.0,0.0).w(), 0.0);
    }

    #[test]
    fn test_a_point_w_is_1() {
        assert_eq!(Tuple::point(0.0,0.0,0.0).w(), 1.0);
    }
}