use std::{
    collections::{BTreeMap, btree_map::Values},
    path::PathBuf,
};

use crate::{geom::Point, io::reader};

#[derive(Default)]
pub struct Nodes {
    pub nodes: BTreeMap<String, Node>,
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
    pub fn iter(&self) -> Values<'_, String, Node> {
        self.nodes.values()
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get(&self, name: &str) -> Option<&Node> {
        self.nodes.get(name)
    }

    pub fn is_terminal_ni(&self, name: &str) -> bool {
        matches!(
            self.nodes.get(name).unwrap().moveable,
            Movable::FixedButOverlapAllowed
        )
    }
    pub async fn read(path: &PathBuf) -> anyhow::Result<Self> {
        let mut reader = reader::TokenReader::new_from_path(path);
        let mut ret = Self {
            nodes: BTreeMap::new(),
        };
        let mut last_node_name = String::new();
        while let Some(token) = reader.next_token()? {
            match token.as_bytes() {
                b"#" | b"UCLA" | b"NumNodes" | b"NumTerminals" => {
                    let _ = reader.swallow_line();
                }
                b"terminal" => {
                    ret.nodes.get_mut(&last_node_name).unwrap().moveable = Movable::Fixed;
                }
                b"terminal_NI" => {
                    ret.nodes.get_mut(&last_node_name).unwrap().moveable =
                        Movable::FixedButOverlapAllowed;
                }
                _ => {
                    let name = token.to_string();
                    let x = reader
                        .next_token()?
                        .map(str::parse::<i64>)
                        .unwrap()
                        .unwrap() as f64;
                    let y = reader
                        .next_token()?
                        .map(str::parse::<i64>)
                        .unwrap()
                        .unwrap() as f64;
                    last_node_name = name.clone();
                    let next_node = Node {
                        name: name.clone(),
                        size: Point { x, y },
                        moveable: Movable::Movable,
                    };
                    ret.nodes.insert(name, next_node);
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
