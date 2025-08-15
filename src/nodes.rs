use std::path::PathBuf;

use crate::{geom::Point, io::reader};

#[derive(Default)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Default)]
pub enum Movable {
    #[default]
    Movable,
    Fixed,
    FixedButOverlapAllowed,
}

#[derive(Debug, Default)]
pub struct Node {
    pub name: String,
    pub size: Point,
    pub moveable: Movable,
}

impl Nodes {
    pub fn iter(&self) -> std::slice::Iter<'_, Node> {
        self.nodes.iter()
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub async fn read(path: &PathBuf) -> anyhow::Result<Self> {
        let mut reader = reader::TokenReader::new_from_path(path);
        let mut ret = Self { nodes: vec![] };
        while let Some(token) = reader.next_token()? {
            match token.as_bytes() {
                b"#" | b"UCLA" | b"NumNodes" | b"NumTerminals" => {
                    let _ = reader.swallow_line();
                }
                b"terminal" => {
                    ret.nodes.last_mut().unwrap().moveable = Movable::Fixed;
                }
                b"terminal_NI" => {
                    ret.nodes.last_mut().unwrap().moveable = Movable::FixedButOverlapAllowed;
                }
                _ => {
                    let name = token.to_string();
                    let x = reader
                        .next_token()?
                        .map(|x| str::parse::<i64>(x))
                        .unwrap()
                        .unwrap() as f64;
                    let y = reader
                        .next_token()?
                        .map(|y| str::parse::<i64>(y))
                        .unwrap()
                        .unwrap() as f64;
                    let next_node = Node {
                        name: name,
                        size: Point { x, y },
                        moveable: Movable::Movable,
                    };
                    ret.nodes.push(next_node);
                }
            }
        }
        Ok(ret)
    }

    pub async fn write_in_plain(&self) {
        self.nodes.iter().for_each(|x| {
            println!("{:?}", x);
        });
    }
}
