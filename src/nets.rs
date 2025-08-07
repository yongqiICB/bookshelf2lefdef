use std::{fs::File, io::BufReader, path::PathBuf};

use crate::{geom::Point, io::reader::TokenReader};

#[derive(Default)]
pub struct Pin {
    pin_name: String,
    instance_name: String,
    offset: Point,
}
#[derive(Default)]
pub struct Net {
    name: String,
    pin: Vec<Pin>,
}

#[derive(Default)]
pub struct Nets {
    nets: Vec<Net>,
}

impl Pin {
    pub async fn read(reader: &mut TokenReader<BufReader<File>>) -> anyhow::Result<Self> {
        let mut res = Self::default();
        res.instance_name = reader.next_token()?.unwrap().to_string();
        res.pin_name = reader.next_token()?.unwrap().to_string();
        assert_eq!(b":", reader.next_token()?.unwrap().as_bytes());
        res.offset = Point::read(reader).await?;
        Ok(res)
    }
}
impl Net {
    pub async fn read(reader: &mut TokenReader<BufReader<File>>) -> anyhow::Result<Self> {
        let mut res = Self::default();
        assert_eq!(b"NetDegree", reader.next_token()?.unwrap().as_bytes());
        assert_eq!(":", reader.next_token()?.unwrap());
        let _ = reader.next_token()?.unwrap().as_bytes(); // number;
        let net_name = reader.next_token()?.unwrap().to_string();
        res.name = net_name;
        while let Some(token) = reader.peek_token()? {
            match token.to_uppercase().as_bytes() {
                b"NETDEGREE" => {
                    break;
                }
                _ => {
                    let pin = Pin::read(reader).await.unwrap();
                    res.pin.push(pin);
                }
            }
        }
        Ok(res)
    }
}

impl Nets {
    pub fn len(&self) -> usize {
        self.nets.len()
    }
    pub async fn read_from_file(file_path: PathBuf) -> anyhow::Result<Self> {
        let mut res = Nets::default();
        let mut reader = crate::io::reader::TokenReader::new_from_path(&file_path);
        while let Some(token) = reader.peek_token()? {
            match token.to_ascii_uppercase().as_bytes() {
                b"UCLA" | b"#" | b"NUMNETS" | b"NUMPINS" => {
                    reader.swallow_line()?;
                }
                b"NETDEGREE" => {
                    let net = Net::read(&mut reader).await?;
                    res.nets.push(net);
                }
                _ => {
                    println!("Unexpected token");
                }
            }
        }
        Ok(res)
    }
}
