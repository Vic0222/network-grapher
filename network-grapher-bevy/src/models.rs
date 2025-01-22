use bevy::prelude::Component;


#[derive(Component, Default, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub label: String,
    pub image_src: Option<String>,
}