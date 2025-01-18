use bevy::prelude::Component;


#[derive(Component, Default, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub label: String,
    pub image_url: Option<String>,
}