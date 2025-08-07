use std::{collections::BTreeMap, path::PathBuf};

use crate::{geom::Point, io::reader::CommonReader, util};


#[derive(Default)]
pub struct Grid {
    num_x: i64,
    num_y: i64,
    num_layer: i64,
}

impl Grid {
    pub async fn read(reader: &mut CommonReader) -> anyhow::Result<Self> {
        let mut res = Grid::default();
        assert_eq!("Grid", reader.next_token()?.unwrap());
        assert_eq!(":", reader.next_token()?.unwrap());
        res.num_x = str::parse::<i64>(reader.next_token()?.unwrap())?;
        res.num_y = str::parse::<i64>(reader.next_token()?.unwrap())?;
        res.num_layer = str::parse::<i64>(reader.next_token()?.unwrap())?;
        Ok(res)
    }
}


type TerminalLayer = BTreeMap<String, i64>;
pub trait TerminalLayerReader {
    fn read(reader: &mut CommonReader) ->  impl std::future::Future<Output = anyhow::Result<TerminalLayer>> + Send;
}
impl TerminalLayerReader for TerminalLayer {
    async fn read(reader: &mut CommonReader) -> anyhow::Result<TerminalLayer> {
        let mut res = TerminalLayer::default();
        reader.next_token()?;
        assert_eq!(":", reader.next_token()?.unwrap());
        let num_terminal: i64 = util::parse(reader.next_token()?.unwrap());
        for _ in 0..num_terminal {
            let name = reader.next_token()?.unwrap().to_string();
            let layer_id = util::parse(reader.next_token()?.unwrap());
            res.insert(name, layer_id);
        }
        Ok(res)
    }
}

type BlockageInfo = BTreeMap<String, Vec<i64>>;
pub trait BlockageInfoReader {
    fn read(reader: &mut CommonReader) -> impl std::future::Future<Output = anyhow::Result<BlockageInfo>> + Send;
}

impl BlockageInfoReader for BlockageInfo {
    async fn read(reader: &mut CommonReader) ->  anyhow::Result<BlockageInfo> {
        let mut res = BlockageInfo::new();
        assert_eq!("NumBlockageNodes", reader.next_token()?.unwrap());
        assert_eq!(":", reader.next_token()?.unwrap());
        let num_blockage: i64 = crate::util::parse(reader.next_token()?.unwrap());
        for _ in 0..num_blockage {
            let terminal_name = reader.next_token()?.unwrap().to_string();
            let mut blockage_on_layer_ids = vec![];
            let num_layer: i64 = crate::util::parse(reader.next_token()?.unwrap());
            for _ in 0..num_layer {
                let layer_id: i64 = crate::util::parse(reader.next_token()?.unwrap());
                blockage_on_layer_ids.push(layer_id);
            }
            res.insert(terminal_name, blockage_on_layer_ids);
        }
        Ok(res)
    }
}

#[derive(Default)]
pub struct Route {
    grid: Grid,
    vertical_capacity: Vec<i64>,
    horizontal_capacity: Vec<i64>,
    min_wire_width: Vec<i64>,
    min_wire_spacing: Vec<i64>,
    via_spacing: Vec<i64>,
    grid_origin: Point,
    tile_size: Point,
    blockage_porosity: i64,
    ni_terminal_to_layer: TerminalLayer, // not in image terminal (fixed pin above M1)
    blockage_info: BlockageInfo,
}

impl Route {
    pub fn blockge_len(&self) -> usize {
        self.blockage_info.len()
    }
    pub fn ni_terminal_len(&self) -> usize {
        self.ni_terminal_to_layer.len()
    }
    pub async fn read(route_path: &PathBuf) -> anyhow::Result<Self> {
        let reader = &mut CommonReader::new_from_path(&route_path);
        let mut res = Self::default();
        while let Some(token) = reader.peek_token()? { 
            match token.to_ascii_uppercase().as_bytes() {
                b"ROUTE" | b"#" => {
                    reader.swallow_line()?;
                }
                b"GRID" => {
                    res.grid = Grid::read(reader).await?;
                }
                b"VERTICALCAPACITY" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    for _ in 0..res.grid.num_layer {
                        res.vertical_capacity.push(crate::util::parse(reader.next_token()?.unwrap()));
                    }
                }
                b"HORIZONTALCAPACITY" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    for _ in 0..res.grid.num_layer {
                        res.horizontal_capacity.push(crate::util::parse(reader.next_token()?.unwrap()));
                    }
                }
                b"MINWIREWIDTH" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    for _ in 0..res.grid.num_layer {
                        res.min_wire_width.push(crate::util::parse(reader.next_token()?.unwrap()));
                    }
                }
                b"MINWIRESPACING" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    for _ in 0..res.grid.num_layer {
                        res.min_wire_spacing.push(crate::util::parse(reader.next_token()?.unwrap()));
                    }
                }
                b"VIASPACING" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    for _ in 0..res.grid.num_layer {
                        res.via_spacing.push(crate::util::parse(reader.next_token()?.unwrap()));
                    }
                }
                b"GRIDORIGIN" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    res.grid_origin = Point::read(reader).await?;
                }
                b"TILESIZE" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    res.tile_size = Point::read(reader).await?;
                }
                b"BLOCKAGEPOROSITY" => {
                    reader.next_token()?;
                    assert_eq!(":", reader.next_token()?.unwrap());
                    res.blockage_porosity = crate::util::parse(reader.next_token()?.unwrap());
                }
                b"NUMNITERMINALS" => {
                    res.ni_terminal_to_layer = TerminalLayer::read(reader).await?;
                }
                b"NUMBLOCKAGENODES" => {
                    res.blockage_info = BlockageInfo::read(reader).await?;
                }
                _ => {
                    panic!("Unknown token: {}", token);
                }
            }
        }
        Ok(res)
    }
}