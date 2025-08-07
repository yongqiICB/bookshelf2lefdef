use crate::{aux::Aux, nets::Nets, nodes::Nodes, pl::Pls};

#[derive(Default)]
pub struct Bookshelf {
    aux: Aux,
    nodes: Nodes,
    nets: Nets,
    pls: Pls
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

        
        Ok(res)
    }
}
