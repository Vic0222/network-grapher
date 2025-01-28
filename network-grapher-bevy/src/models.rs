use bevy::prelude::Component;


#[derive(Component, Default, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub label: String,
    pub image_src: Option<String>,
    //pub parent_type: u8, //0 = left side of the graph, 1 = right side of the graph
    pub graph_x: f32, 
    pub graph_y: f32, 
}