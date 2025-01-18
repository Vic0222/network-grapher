use bevy::{math::Vec2, prelude::{Component, Deref, Resource}};

#[derive(Resource)]
pub struct MouseRightButtonPressed(pub bool);