use std::f32::consts::PI;
use bevy::prelude::*;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(animate_light_direction)
        .run();
}
