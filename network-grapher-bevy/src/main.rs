mod components;
mod models;

use bevy::{color::palettes::css::*, input::mouse::{MouseButtonInput, MouseMotion}, prelude::*};
use bevy_prototype_lyon::prelude::*;
use petgraph:: Graph;

use components::MouseRightButtonPressed;
use models::Node;

fn main() {
    
    let mut binding = App::new();
    let app = binding
        .add_plugins((DefaultPlugins,ShapePlugin))
        .add_systems(Startup, (setup, setup_graph, draw_nodes, draw_edges).chain())
        .add_systems(Update, (handle_camera_move_using_mouse).chain());

        app.run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.insert_resource(MouseRightButtonPressed(false));
}

const SIZE: f32 = 100.0;
const ARROW_THICKNESS: f32 = 3.0;
const CAM_LERP_FACTOR: f32 = 1.;

#[derive(Resource, Deref)]
pub struct GraphResource(pub Graph<Node, i32>);
fn setup_graph(mut commands: Commands) {
    let node1 = Node { id: 1, label: "Father".to_string(), image_url: Some( "https://placehold.co/100".to_string()) };
    let node2 = Node { id: 2, label: "Mother".to_string(), image_url: Some( "https://placehold.co/100".to_string()) };
    let node3 = Node { id: 3, label: "Child".to_string(), image_url: Some( "https://placehold.co/100".to_string()) };
    let mut graph = Graph::<Node, i32>::new();
    
    let a = graph.add_node(node1);
    let b = graph.add_node(node2);
    let c = graph.add_node(node3);
    
    //g.add_edge(a, b, 0);
    graph.add_edge(a, c, 0);
    graph.add_edge(b, c, 0);
    
    commands.insert_resource(GraphResource(graph));
}

// Handle user mouse input for panning the camera around
fn handle_camera_move_using_mouse(
    mut button_events: EventReader<MouseButtonInput>,
    mut motion_events: EventReader<MouseMotion>,
    mut mouse_pressed: ResMut<MouseRightButtonPressed>,
    mut camera: Query<&mut Transform, With<Camera2d>>
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    // Store left-pressed state in the MousePressed resource
    for button_event in button_events.read() {
        
        match button_event.button {
            MouseButton::Right => {
                *mouse_pressed = MouseRightButtonPressed(button_event.state.is_pressed());
            },
            _ => continue,
        }
        
    }
    
    // If the mouse is not pressed, just ignore motion events
    if !mouse_pressed.0 {
        return;
    }
    
    let (dx, dy) = motion_events
        .read()
        .fold((0.0, 0.0), |(acc_x, acc_y), mouse_motion| (acc_x + mouse_motion.delta.x, acc_y + mouse_motion.delta.y));

    //multiply by -1 to invert becuase we move the camera itself not the object
    let x = camera.translation.x + (dx * -1.0); 

    //we don't reverse this one as the value is already reversed from the mouse
    let y = camera.translation.y + dy;

    //println!("X: {} , Y: {}, dX {}, dY {}", x, y, dx, dy);
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = camera
        .translation
        .lerp(direction,  CAM_LERP_FACTOR);

       // println!("Box X: {} , Y: {}, Z: {}", camera.translation.x, camera.translation.y, camera.translation.z);
}

fn draw_nodes(mut commands:  Commands, graph_resource: Res<GraphResource>) {
    let mut i = 0.0;
    let spacing = 100.0;

    //draw_node(&mut commands, i, spacing, node1);
    graph_resource.0.raw_nodes().iter().for_each(|node| {
        draw_node(&mut commands, i, spacing, node);
        i += 1.0;
    });
    
}

fn draw_node( commands: &mut  Commands<'_, '_>, i: f32, spacing: f32, node: &petgraph::graph::Node<Node>) {
    let entity: Entity = commands.spawn_empty().id();
    let color = Color::hsl(360. * i as f32 / 3 as f32, 0.95, 0.7);
    let y = if i % 2.0 == 0.0 { 1.0 } else { 0.0 };
    commands.entity(entity)
    .insert((Sprite {
        color: color,
        custom_size: Some(Vec2::new(SIZE, SIZE)),
        ..default()
        }, Transform::from_xyz((spacing + SIZE )* i, (spacing + SIZE )* y, 0.0),
        node.weight.clone()
    ))
    .with_child((
        Text2d::new(node.weight.label.to_string()),
        Transform::from_xyz(0.0, -70.0 , 0.0)
    ));
}

fn draw_edges(mut commands: Commands,
    graph_resource: Res<GraphResource>,
    query: Query<(&Node, &Transform), With<Node>>) {
    
    
    graph_resource.0.raw_edges().iter().for_each(|edge| {
        let arrow = commands.spawn_empty().id();
        let source_node = query.iter().find(|(n, _) |{
            match graph_resource.0.node_weight(edge.source()) {
                Some(n2) => n.id == n2.id,
                _ => false,
            }
        });

        let target_node = query.iter().find(|(n, _) |{
            match graph_resource.0.node_weight(edge.target()) {
                Some(n2) => n.id == n2.id,
                _ => false,
            }
        });
        if source_node.is_some() && target_node.is_some() {
            draw_arrow(&mut commands, arrow, source_node.unwrap().1, target_node.unwrap().1);
        }
        
    });
    
}

fn draw_arrow(commands: &mut Commands<'_, '_>, arrow: Entity, source_node: &Transform, target_node: &Transform) {
    let length = source_node.translation.distance(target_node.translation);
    
    let length = length - (SIZE/2.0) - ARROW_THICKNESS;

    let mut transform = source_node.clone();
    
    let translation = transform.translation.xy();
    let to_target = (translation - target_node.translation.xy()).normalize();
    let rotate_to_target = Quat::from_rotation_arc(Vec3::NEG_Y, to_target.extend(0.));
        
        
    transform.rotation = rotate_to_target;
        
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0.0 -ARROW_THICKNESS, 0.));
    path_builder.line_to(Vec2::new(0.0 -ARROW_THICKNESS, length));
    path_builder.line_to(Vec2::new(0.0 - ARROW_THICKNESS - ARROW_THICKNESS, length));
    path_builder.line_to(Vec2::new(0.0, length + ARROW_THICKNESS));
    // center
    path_builder.line_to(Vec2::new(ARROW_THICKNESS + ARROW_THICKNESS , length));
    path_builder.line_to(Vec2::new( ARROW_THICKNESS , length));
    path_builder.line_to(Vec2::new( ARROW_THICKNESS , 0.));
    path_builder.close();
    let path = path_builder.build();

    transform.translation.z = 1.0;
    commands.entity(arrow).insert((
        ShapeBundle {
            path,
            transform: transform,
            visibility: default(),
            ..default()
        },
        Fill::color(WHITE),
        Stroke::new(WHITE, 1.0),
    ));
}