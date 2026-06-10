use petgraph::graph::UnGraph;
use petgraph::visit::Bfs;
use std::collections::HashMap;

// ── Función BFS que devuelve el camino con menos saltos ────────────────────────
fn bfs_ruta(
    grafo: &UnGraph<&str, u32>,
    origen: petgraph::graph::NodeIndex,
    destino: petgraph::graph::NodeIndex,
) -> Option<Vec<petgraph::graph::NodeIndex>> {
    let mut padres: HashMap<petgraph::graph::NodeIndex, petgraph::graph::NodeIndex> =
        HashMap::new();

    let mut bfs = Bfs::new(grafo, origen);

    while let Some(nodo) = bfs.next(grafo) {
        if nodo == destino {
            let mut camino = vec![destino];
            let mut actual = destino;
            while actual != origen {
                actual = *padres.get(&actual).unwrap();
                camino.push(actual);
            }
            camino.reverse();
            return Some(camino);
        }
        for vecino in grafo.neighbors(nodo) {
            padres.entry(vecino).or_insert(nodo);
        }
    }

    None
}

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

  let ruta = bfs_ruta(&grafo, san_salvador, la_union);

    match ruta {
        Some(camino) => {
            let nombres: Vec<&str> = camino.iter().map(|&idx| grafo[idx]).collect();
            let km_total: u32 = camino
                .windows(2)
                .map(|par| {
                    let arista = grafo.find_edge(par[0], par[1]).unwrap();
                    grafo[arista]
                })
                .sum();
            println!(
                "Ruta encontrada ({} escalas, {} km en total):",
                nombres.len() - 1,
                km_total
            );
            println!("  {}", nombres.join(" → "));
            println!("\n  Desglose:");
            for par in camino.windows(2) {
                let arista = grafo.find_edge(par[0], par[1]).unwrap();
                println!(
                    "    {} → {}  ({} km)",
                    grafo[par[0]], grafo[par[1]], grafo[arista]
                );
            }
        }
        None => println!("No existe ruta entre las ciudades."),
    }

}