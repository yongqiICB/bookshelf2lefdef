use crate::{geom::Point, parser::Bookshelf};



#[derive(Debug, Default)]
pub struct Rows {
    site_name: String,
    rows: Vec<crate::scl::Row>,
}

impl Rows {
    pub fn build(bookshelf: &Bookshelf, site_name: String) -> Self {
        let mut res = Self {
            site_name,
            ..Default::default()
        };
        
        for row in bookshelf.scl.iter() {
            res.rows.push(row.clone());
        }
        res
    }

    pub fn write(&self) -> String {
        let mut res = String::new();
        for (iter, row) in self.rows.iter().enumerate() {
            let orientation = match iter % 2 {
                0 => "FS",
                1 => "N",
                _ => panic!("iter % 2 < 2"),
            };
            res += &format!("\n ROW CORE_ROW_{} {} {} {} {} DO {} BY 1 STEP {} 0 ;", 
                iter,
                self.site_name,
                row.subrow_origin * 1000,
                row.coordinate * 1000,
                orientation,
                row.num_sites,
                row.site_width * 1000,
            );
        }
        res
    }
}