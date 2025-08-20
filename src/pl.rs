use crate::{geom::Point, io::reader::TokenReader};
use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

pub struct Pl {
    pub name: String,
    pub place: Point,
    pub orientation: String,
    pub r#type: Type,
}

#[derive(Default)]
pub enum Type {
    #[default]
    Movable,
    Fixed,
    FixedNotInImage,
}
impl Pl {
    pub async fn read(reader: &mut TokenReader<BufReader<File>>) -> anyhow::Result<Self> {
        let name = reader.next_token()?.unwrap().to_string();
        let place = Point::read(reader).await?;
        assert_eq!(":", reader.next_token()?.unwrap());
        let orientation = reader.next_token()?.unwrap().to_string();
        let r#type = if let Some(next_token) = reader.peek_token()? {
            if next_token.to_ascii_uppercase().as_bytes() == b"/FIXED" {
                reader.next_token()?;
                Type::Fixed
            } else if next_token.to_ascii_uppercase().as_bytes() == b"/FIXED_NI" {
                reader.next_token()?;
                Type::FixedNotInImage
            } else {
                Type::Movable
            }
        } else {
            Type::Movable
        };
        Ok(Pl {
            name,
            place,
            orientation,
            r#type,
        })
    }
}

#[derive(Default)]
pub struct Pls {
    pls: BTreeMap<String, Pl>,
}

impl Pls {
    pub fn iter(&self) -> std::collections::btree_map::Values<'_, String, Pl> {
        self.pls.values()
    }
    pub fn len(&self) -> usize {
        self.pls.len()
    }

    pub  fn get(&self, name: &str) -> Option<&Pl> {
        self.pls.get(name)
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
                    let place = Pl::read(&mut reader).await?;
                    res.pls.insert(place.name.clone(), place);
                }
            }
        }
        Ok(res)
    }
}
