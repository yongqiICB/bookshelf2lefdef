use core::panic;
use std::{default, fs::File, io::BufReader, path::PathBuf};

use bytes::buf::Reader;

use crate::{geom::Point, io::reader::TokenReader};

pub struct Pl {
    pub name: String,
    pub place: Point,
    pub orientation: String,
    pub fixed: bool,
}

impl Pl {
    pub async fn read(reader: &mut TokenReader<BufReader<File>>) -> anyhow::Result<Self> {
        let name = reader.next_token()?.unwrap().to_string();
        let place = Point::read(reader).await?;
        assert_eq!(":", reader.next_token()?.unwrap());
        let orientation = reader.next_token()?.unwrap().to_string();
        let fixed = if let Some(next_token) = reader.peek_token()? {
            if next_token.to_ascii_uppercase().as_bytes() == b"/FIXED" {
                reader.next_token()?;
                true
            } else if next_token.to_ascii_uppercase().as_bytes() == b"/FIXED_NI"{
                reader.next_token()?;
                true
            } else {
                false
            }
        } else {
            false
        };
        Ok(Pl {
            name,
            place,
            orientation,
            fixed,
        })
    }
}

#[derive(Default)]
pub struct Pls {
    pls: Vec<Pl>
}

impl Pls {
    pub fn len(&self) -> usize {
        self.pls.len()
    }
    pub async fn read_from_file(file_path: &PathBuf) -> anyhow::Result<Self> {
        let mut res = Self::default();
        let mut reader = crate::io::reader::TokenReader::new_from_path(file_path);
        while let Some(token) = reader.peek_token()? {
            match token.to_ascii_uppercase().as_bytes() {
                b"UCLA" | b"#" => {
                    reader.swallow_line()?;
                }
                _ => {
                    let net = Pl::read(&mut reader).await?;
                    res.pls.push(net);
                }
            }
        }
        Ok(res)
    }
}