use crate::{
    lefdef::writer::{CutLayer, OverlapLayer, RoutingLayer},
    parser::Bookshelf,
};

pub struct TechLef {
    pub layer: Vec<RoutingLayer>,
}

impl TechLef {
    pub async fn build(bookshelf: &Bookshelf) -> anyhow::Result<Self> {
        let routing_layers = RoutingLayer::build_routing_layers(bookshelf).await?;
        Ok(Self {
            layer: routing_layers,
        })
    }

    pub async fn write(&self) {
        println!("{}", OverlapLayer::format_a_default_one());
        for (id, layer) in self.layer.iter().enumerate() {
            println!("{}", layer.format());
            if id != self.layer.len() - 1 {
                println!(
                    "{}",
                    CutLayer::format_a_default_one(format!("CUT{}", id + 1))
                );
            }
        }
    }
}
