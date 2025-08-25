use log::info;

use crate::{geom::{Point, Rect}, parser::Bookshelf};

enum Direction {
    X,
    Y
}
pub struct Track {
    direction: Direction,
    start: i64,
    num_tracks: i64,
    step: i64,
    layer: String,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::X => write!(f, "X"),
            Direction::Y => write!(f, "Y"),
        }
    }
}
impl Track {
    pub fn write_all(&self) -> String {
        format!("\n TRACKS {} {} DO {} STEP {} LAYER {} ;",
            self.direction,
            self.start,
            self.num_tracks,
            self.step,
            self.layer
        )
    }
}
pub struct Tracks {
    tracks: Vec<Track>
}

impl Tracks {
    pub fn write(&self) -> String {
        let mut res = String::new();
        for track in self.tracks.iter() {
            res += &track.write_all();
        }
        res
    }
    pub fn build(bookshelf: &Bookshelf) -> Self {
        let mut tracks = vec![];
        let layer_count = bookshelf.route.min_wire_width.len();
        let die_area = {
            let ll = bookshelf.route.grid_origin;
            let ur = {
                let urx = ll.x + bookshelf.route.tile_size.x * bookshelf.route.grid.num_x as f64;
                let ury = ll.y + bookshelf.route.tile_size.y * bookshelf.route.grid.num_y as f64;
                Point {x: urx, y: ury}
            };
            Rect { ll, ur }
        };
        let core_area = {
            let ll = {
                let llx = bookshelf.scl.iter().map(|x| {
                    x.subrow_origin
                }).min().unwrap();
                let lly = bookshelf.scl.iter().map(|x| {
                    x.coordinate
                }).min().unwrap();
                Point {x: llx as f64, y: lly as f64}
            };
            let ur = {
                let urx = bookshelf.scl.iter().map(|x| {
                    x.subrow_origin + x.num_sites * x.site_width
                }).max().unwrap();
                let ury = bookshelf.scl.iter().map(|x| {
                    x.coordinate + x.height
                }).max().unwrap();
                Point {
                    x: urx as f64,
                    y: ury as f64,
                }
            };
            Rect { ll, ur }
        };
        info!("DIEAREA: {:?}", die_area);
        info!("COREAREA: {:?}", core_area);
        for layer_id in 0..layer_count {
            let pitch = bookshelf.route.min_wire_spacing[layer_id] * 1000 + bookshelf.route.min_wire_width[layer_id] * 1000;
            { // HANDLE X
                let start = (die_area.ll.x as i64 * 1000) + pitch / 2;
                let num_tracks = (die_area.ur.x as i64 * 1000 - start) / pitch;
                let step = pitch;
                let layer = format!("metal{}", layer_id + 1);
                tracks.push(Track {
                    direction: Direction::X,
                    start,
                    num_tracks,
                    step,
                    layer
                });
            }

            { // HANDLE Y
                let start = (die_area.ll.y as i64 * 1000) + pitch / 2;
                let num_tracks = (die_area.ur.y as i64 * 1000 - start) / pitch;
                let step = pitch;
                let layer = format!("metal{}", layer_id + 1);
                tracks.push(Track {
                    direction: Direction::Y,
                    start,
                    num_tracks,
                    step,
                    layer
                });
            }
        }
        Self {tracks}
    }
}