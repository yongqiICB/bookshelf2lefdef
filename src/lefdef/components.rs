use crate::parser::Bookshelf;

pub struct Components {
    num: i64,
    to_print: String
}

impl Components {
    pub fn build(bookshelf: &Bookshelf) -> Self {
        let mut res = String::new();
        let mut num_comp = 0;
        for node in bookshelf.nodes.iter() {
            let moveable = match node.moveable {
                crate::nodes::Movable::Movable => "PLACED",
                crate::nodes::Movable::Fixed => "FIXED",
                crate::nodes::Movable::FixedButOverlapAllowed => continue,
            };
            let place = &bookshelf.pls.get(&node.name).unwrap().place.clone();
            num_comp += 1;
            res += &format!("\n- {} {} + {} ( {} {} ) N ;",
                node.name,
                node.name,
                moveable,
                (place.x as i64) * 1000,
                (place.y as i64) * 1000,
            );
        }
        Self{to_print:res, num: num_comp}
    }

    pub fn write(&self) -> String {
        let mut res = format!("\nCOMPONENTS {} ;", self.num);
        res += &self.to_print;
        res += &format!("\nEND COMPONENTS");
        res        
    }
}