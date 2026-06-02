mod grid;

use bevy::prelude::*;

use grid::Edges;

fn main() {
    //  App::new().add_systems(Update, hello_world_system).run();
    hello_world_system();
}

fn hello_world_system() {
    let edges = Edges::triangle_hex(1);
    println!("{:?}", edges.edges);
    let edges = Edges::triangle_hex(2);
    println!("{:?}", edges.edges);
    let edges = Edges::triangle_hex(3);
    println!("{:?}", edges.edges);
}
