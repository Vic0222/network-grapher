use bevy::prelude::Component;
use serde::{Deserialize, Serialize};


#[derive(Component, Default, Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: i32,
    pub label: String,
    pub image_src: Option<String>,
    //pub parent_type: u8, //0 = left side of the graph, 1 = right side of the graph
    pub graph_x: f32, 
    pub graph_y: f32, 
}