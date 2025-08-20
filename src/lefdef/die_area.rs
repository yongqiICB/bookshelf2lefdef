use crate::{geom::{Point, Rect}, parser::Bookshelf};

pub struct DieArea {
    die_area: Rect
}
impl DieArea {
    pub fn build(bookshelf: &Bookshelf) -> Self {
        let ll = bookshelf.route.grid_origin;
        let ur = {
            let urx = ll.x + bookshelf.route.tile_size.x * bookshelf.route.grid.num_x as f64;
            let ury = ll.y + bookshelf.route.tile_size.y * bookshelf.route.grid.num_y as f64;
            Point {x: urx, y: ury}
        };
        Self {die_area: Rect { ll, ur }}
    }
    pub fn write(&self) -> String {
        format!("\nDIEAREA ( {} {} ) ( {} {} ) ;",
            (self.die_area.ll.x * 1000.0) as i64,
            (self.die_area.ll.y * 1000.0) as i64,
            (self.die_area.ur.x * 1000.0) as i64,
            (self.die_area.ur.y * 1000.0) as i64,
        )
    }
}