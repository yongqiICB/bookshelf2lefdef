use std::{fs::File, io::BufReader};

use crate::io::reader::TokenReader;

#[derive(Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub async fn read(reader: &mut TokenReader<BufReader<File>>) -> anyhow::Result<Self> {
        let mut res = Self::default();
        res.x = str::parse::<f64>(reader.next_token()?.unwrap()).unwrap();
        res.y = str::parse::<f64>(reader.next_token()?.unwrap()).unwrap();
        Ok(res)
    }
}
