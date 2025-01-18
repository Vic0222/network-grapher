use bevy::{color::palettes::css::*, prelude::*, utils::HashSet};
use bevy_prototype_lyon::prelude::*;
use petgraph::{ csr::DefaultIx, graph::NodeIndex, Graph};


fn main() {
    
    let mut binding = App::new();
    let app = binding
        .add_plugins((DefaultPlugins,ShapePlugin))
        .add_systems(Startup, (setup, setup_graph, draw_nodes, draw_edges).chain());

        app.run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Node {
    pub id: i32
}

const SIZE: f32 = 100.0;
const ARROW_THICKNESS: f32 = 3.0;

#[derive(Resource, Deref)]
pub struct GraphResource(pub Graph<Node, i32>);
fn setup_graph(mut commands: Commands) {
    let node1 = Node { id: 1};
    let node2 = Node { id: 2};
    let node3 = Node { id: 3};
    let mut g = Graph::<Node, i32>::new();
    
    let a = g.add_node(node1);
    let b = g.add_node(node2);
    let c = g.add_node(node3);
    
    //g.add_edge(a, b, 0);
    g.add_edge(a, c, 0);
    g.add_edge(b, c, 0);
    
    commands.insert_resource(GraphResource(g));
}

fn draw_nodes(mut commands:  Commands, graph_resource: Res<GraphResource>) {
    let mut i = 0.0;
    let spacing = 100.0;
    let drawn_nodes = HashSet::<i32>::new();
    let node1 = graph_resource.0.raw_nodes().first().expect("expected atleast 1 node.");

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
    commands.entity(entity).insert((Sprite {
        color: color,
        custom_size: Some(Vec2::new(SIZE, SIZE)),
        ..default()
    }, Transform::from_xyz((spacing + SIZE )* i, (spacing + SIZE )* y, 0.0),
    node.weight.clone()));
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