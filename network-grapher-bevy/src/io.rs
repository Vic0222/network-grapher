use std::{collections::HashMap, env, fs::{self, File}, io::Read};

use anyhow::Result;
use petgraph::Graph;
use serde::{Deserialize, Serialize};


use crate::models::Node;

pub fn save_to_file(graph: &Graph::<Node, i32>) -> Result<()>{
    let raw_nodes: Vec<Node> = graph.raw_nodes().iter().map(|n| {
        n.weight.clone()
    }).collect();

    let raw_edges: Vec<Edge> = graph.raw_edges().iter().map(|e| {
        let src_id = graph.node_weight(e.source())?.id;
        let target_id = graph.node_weight(e.target())?.id;
        Some(Edge {src_id, target_id, weight: e.weight})
    }).filter_map(|e| e).collect();
    let save_data = SaveData {
        nodes: raw_nodes,
        edges: raw_edges
    };

    let json = serde_json::to_string(&save_data)?;
    fs::write("./graph.json", json)?;

    Ok(())
}

pub fn load_from_file() -> Result<Graph<Node, i32>> {
    let path = env::current_dir()?;
    
    let file_path = format!("{}\\{}", path.display(), "graph.json");
    
    println!("The current file is {}", file_path);
    let json :String = fs::read_to_string(file_path)?;
    let save_data: SaveData = serde_json::from_str(&json)?;
    let mut graph = Graph::<Node, i32>::new();
    let mut nodex_indexs: HashMap<i32, petgraph::prelude::NodeIndex>    = HashMap::new();
    for node in save_data.nodes {
        let node_id = node.id;
        let node_index =graph.add_node(node);
        nodex_indexs.insert(node_id, node_index);
    }
    
    for edge in save_data.edges {
        let index_a = nodex_indexs.get(&edge.src_id).ok_or(anyhow::anyhow!("Source not found!"))?;
        let index_b = nodex_indexs.get(&edge.target_id).ok_or(anyhow::anyhow!("Target not found"))?;
        graph.add_edge(index_a.clone(), index_b.clone(), edge.weight);
    }

    println!("Graph loaded");
    Ok(graph)
}

#[derive(Serialize, Deserialize)]
struct Edge {
    src_id: i32,
    target_id: i32,
    weight: i32
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}