use bevy::prelude::*;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .run();
}

#[derive(Component)]
struct Rotator;

fn rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotator>>
){
    for mut transform in &mut query {
        transform.rotate_x(3.0 * time.delta_seconds());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 2.0}));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    commands
        .spawn(
            (PbrBundle {
                mesh: cube_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            Rotator,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_xyz(0.0,0.0, 3.0),
                ..default()
            });
        });
    
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}