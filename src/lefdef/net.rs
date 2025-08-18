use std::collections::BTreeMap;

use crate::lefdef::writer::Macros;

#[derive(Debug, Clone)]
pub enum Node {
    Pin(String),
    InstancePin(String, String),
}
pub struct Net {
    pub name: String,
    pub nodes: Vec<Node>,
}

pub struct Nets(BTreeMap<String, Net>);

impl Nets {
    pub fn build_net(macros: &Macros) -> Self {
        Self(
            macros
                .net_to_nodes
                .iter()
                .map(|(net_name, nodes)| {
                    (
                        net_name.clone(),
                        Net {
                            name: net_name.clone(),
                            nodes: nodes.clone(),
                        },
                    )
                })
                .collect(),
        )
    }

    pub fn write(&self) -> String {
        let mut res = String::new();
        let net_len = self.0.len();
        res += &format!("\nNETS {} ;", net_len);
        for (net_name, net) in self.0.iter() {
            res += &format!("\n- {}", net_name);
            for node in net.nodes.iter() {
                match node {
                    Node::Pin(pin_name) => {
                        res += &format!(" ( PIN {} )", pin_name);
                    }
                    Node::InstancePin(inst_name, pin_name) => {
                        res += &format!(" ( {} {} )", inst_name, pin_name);
                    }
                }
            }
            res += " + USE SIGNAL ;";
        }
        res
    }
}
