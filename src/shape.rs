use std::path::PathBuf;

use crate::{geom::Rect, io::reader::CommonReader};

#[derive(Default)]
pub struct Shape {
    pub shape_name: String,
    pub rect: Rect,
}

impl Shape {
    pub async fn read(reader: &mut CommonReader) -> anyhow::Result<Self> {
        let name = reader.next_token()?.unwrap().to_string();
        let shape = Rect::read_by_lowerleft_width_height(reader).await?;
        Ok(Self {
            shape_name: name,
            rect: shape
        })
    }
}

#[derive(Default)]
pub struct NodeShape {
    node_name: String,
    shape: Vec<Shape>,
}

impl NodeShape {
    pub async fn read(reader: &mut CommonReader) -> anyhow::Result<Self> {
        let mut res = NodeShape::default();
        res.node_name = reader.next_token()?.unwrap().to_string();
        reader.expect(":")?;
        let num_shape = crate::util::parse(reader.next_token()?.unwrap());
        for _ in 0..num_shape {
            res.shape.push(Shape::read(reader).await.unwrap());
        }
        Ok(res)
    }
}



#[derive(Default)]
pub struct Shapes {
    shapes: Vec<NodeShape>
}

impl Shapes {
    pub fn len(&self) -> usize {
        self.shapes.len()
    }
    pub async fn read_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let mut res = Self::default();
        let mut reader = CommonReader::new_from_path(path);
        while let Some(token) = reader.peek_token()? {
            match token.to_ascii_uppercase().as_bytes() {
                b"SHAPES" | b"#" | b"NUMNONRECTANGULARNODES" => {
                    reader.swallow_line()?;
                }
                _ => {
                    res.shapes.push(NodeShape::read(&mut reader).await?);
                }
            }
        }
        Ok(res)

    }
}

