use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{aux::Aux, nets::{Net, Nets, Pin}, nodes::Nodes, pl::Pls, route::Route, scl::Scl, shape::Shapes};

#[derive(Default)]
pub struct Bookshelf {
    pub aux: Aux,
    pub nodes: Nodes,
    pub nets: Nets,
    pub pls: Pls,
    pub scl: Scl,
    pub route: Route,
    pub shape: Shapes,
}

impl Bookshelf {
    pub async fn build_from_aux(aux: Aux) -> anyhow::Result<Self> {
        let mut res = Self::default();
        if let Some(nodes_path) = aux.nodes.as_ref() {
            res.nodes = Nodes::read(nodes_path).await?;
            println!("Read {} nodes", res.nodes.len())
        };
        if let Some(net_path) = aux.nets.as_ref() {
            res.nets = Nets::read_from_file(net_path.clone()).await?;
            println!("Read {} nets", res.nets.len());
        }

        if let Some(pl_path) = aux.pl.as_ref() {
            res.pls = Pls::read_from_file(pl_path).await?;
            println!("Read {} pls", res.pls.len());
        }

        if let Some(scl_path) = aux.scl.as_ref() {
            res.scl = Scl::read_from_file(scl_path).await?;
            println!("Read {} rows", res.scl.len());
        }

        if let Some(route_path) = aux.route.as_ref() {
            res.route = Route::read(route_path).await?;
            println!("Read Route\
                \n  {} Not in Image Terminals (akin metals on layer higher than M1. Like VSS in real design).\
                \n  {} Routing Blockage(routing blockage in different metal, which is saved in `res.route.blockage_info`)",
                res.route.ni_terminal_len(),
                res.route.blockge_len(),
            );
        }

        if let Some(shape_path) = aux.shapes.as_ref() {
            res.shape = Shapes::read_from_file(shape_path).await?;
            println!("Read {} shape.", res.shape.len());
        }

        if let Some(_) = aux.wts.as_ref() {
            println!("Unimplemented wts!");
        }
        Ok(res)
    }

    pub async fn parse(&self) {
        println!("Start parsing ...");
        let mut instances = BTreeMap::new();
        let mut pins = BTreeMap::new();
        let mut sizes: BTreeSet<(i64, i64)> = BTreeSet::new();
        self.nodes.nodes.iter().for_each(|node| {
            match node.moveable {
                crate::nodes::Movable::Movable | crate::nodes::Movable::Fixed => {
                    instances.insert(node.name.clone(), node.size);
                    assert_eq!(0f64, node.size.x.floor() - node.size.x, "node.size.x is not integer {}", node.size.x);
                    assert_eq!(0f64, node.size.y.floor() - node.size.y, "node.size.y is not integer {}", node.size.y);
                    sizes.insert((node.size.x as i64, node.size.y as i64));
                },
                crate::nodes::Movable::FixedButOverlapAllowed => {
                    pins.insert(node.name.clone(), node.size);
                    assert_eq!(0f64, node.size.x.floor() - node.size.x, "node.size.x is not integer {}", node.size.x);
                    assert_eq!(0f64, node.size.y.floor() - node.size.y, "node.size.y is not integer {}", node.size.y);
                    sizes.insert((node.size.x as i64, node.size.y as i64));
                },
            }
        });
        println!(
            "Splited Nodes into Components and Pins\
            \n  Pins: {}\
            \n  Instances: {}\
            \n  Number of different sizes: {}.",
            pins.len(), instances.len(), sizes.len()
        );
        let mut net_in_instances = BTreeMap::new();
        self.nets.iter().for_each(|net| {
            net.pin.iter().for_each(|pin| {
                net_in_instances
                    .entry(pin.instance_name.clone())
                    .or_insert(vec![]).push(net.name.clone());
            }); 
        });
    }
}
