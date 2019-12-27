use textwrap;

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

use std::fmt;

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s_values: Vec<_> = self.body.iter().map(|c| format!("{}", c)).collect();
        let body = s_values.join(" ");
        write!(f, "{}\n{}\n", self.header(), textwrap::fill(&body, 70))
    }
}
