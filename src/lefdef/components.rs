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
            num_comp += 1;
            res += &format!("\n- {} {} + {} ( {} {} ) N ;",
                node.name,
                node.name,
                moveable,
                (node.size.x as i64) * 1000,
                (node.size.y as i64) * 1000,
            );
        }
        Self{to_print:res, num: num_comp}
    }

    pub fn write(&self) -> String {
        let mut res = format!("\nCOMPONENTS {}", self.num);
        res += &self.to_print;
        res        
    }
}