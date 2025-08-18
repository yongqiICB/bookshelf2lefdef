use std::{fs::File, io::BufReader, ops::AddAssign};

use crate::io::reader::{CommonReader, TokenReader};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Point {
    pub async fn read(reader: &mut TokenReader<BufReader<File>>) -> anyhow::Result<Self> {
        let mut res = Self::default();
        res.x = str::parse::<f64>(reader.next_token()?.unwrap()).unwrap();
        res.y = str::parse::<f64>(reader.next_token()?.unwrap()).unwrap();
        Ok(res)
    }
}

#[derive(Debug, Default)]
pub struct Rect {
    pub ll: Point,
    pub ur: Point,
}

impl Rect {
    pub async fn read_by_lowerleft_width_height(reader: &mut CommonReader) -> anyhow::Result<Self> {
        let ll = Point::read(reader).await?;
        let mut ur = Point::read(reader).await?;
        ur += ll;
        Ok(Self { ll, ur })
    }
}
