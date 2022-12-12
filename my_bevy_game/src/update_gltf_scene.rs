use bevy::prelude::*;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_scene_entities)
        .run();
}

#[derive(Component)]
struct MovedScene;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.05, 0.9, 1.5)
            .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });

    // Spawn the scene as achild of this entity at the given transform
    commands.spawn((
        SceneBundle {
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        scene: asset_server.load("models/FlightHelmet/glTF/FlightHelmet.gltf#Scene0"),
        ..default()
        },
    ));

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/FlightHelmet/glTF/FlightHelmet.gltf#Scene0"),
            ..default()
        },
        MovedScene,
    ));
}

fn move_scene_entities(
    time: Res<Time>,
    moved_scene: Query<Entity, With<MovedScene>>,
    children: Query<&Children>,
    mut transform: Query<&mut Transform>,
){
    for moved_scene_entity in &moved_scene {
        let mut offset = 0.;
        for entity in children.iter_descendants(moved_scene_entity) {
            if let Ok(mut transform) = transform.get_mut(entity) {
                transform.translation = Vec3::new(
                    offset * time.elapsed_seconds().sin() / 20.,
                    0.,
                    time.elapsed_seconds().cos() / 20.,
                );
                offset += 1.0;
            }
        }
    }
}