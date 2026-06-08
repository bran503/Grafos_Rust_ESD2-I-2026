use petgraph::graph::UnGraph;
use petgraph::visit::Bfs;
use std::collections::HashMap;

fn main() {



 let mut grafo: UnGraph<&str, u32> = UnGraph::new_undirected();

    let san_salvador = grafo.add_node("San Salvador");
    let santa_ana    = grafo.add_node("Santa Ana");
    let san_miguel   = grafo.add_node("San Miguel");
    let sonsonate    = grafo.add_node("Sonsonate");
    let usulutan     = grafo.add_node("Usulután");
    let la_union     = grafo.add_node("La Unión");
    let ahuachapan   = grafo.add_node("Ahuachapán");
    let san_vicente  = grafo.add_node("San Vicente");

    grafo.add_edge(san_salvador, santa_ana,   63);
    grafo.add_edge(san_salvador, sonsonate,   64);
    grafo.add_edge(san_salvador, san_vicente, 57);
    grafo.add_edge(santa_ana,    sonsonate,   37);
    grafo.add_edge(santa_ana,    ahuachapan,  40);
    grafo.add_edge(san_vicente,  usulutan,    55);
    grafo.add_edge(usulutan,     san_miguel,  75);
    grafo.add_edge(san_miguel,   la_union,    42);


}