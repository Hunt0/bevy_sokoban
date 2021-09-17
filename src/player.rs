use bevy::prelude::*;
use crate::map::Map;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_player.system().after("load_map"))
            .add_system(player_movement_system.system());
    }
}

struct Player {
    speed: f32,
}

// pub fn position_to_translation(
//     map: &Res<Map>,
//     tile_size: &Res<TileSize>,
//     position: &Position,
//     z: f32,
// ) -> Transform {
//     Transform::from_translation(Vec3::new(
//         (position.x as f32 - (map.width - 1) as f32 / 2.0) * tile_size.0,
//         (-(position.y as f32) + (map.height - 1) as f32 / 2.0) * tile_size.0,
//         z,
//     ))
// }

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let texture_handle = asset_server.load("images/player.png");
    let position = map.entity_positions.get(&'P').unwrap();

    println!("PLAYER POSITION: {}", position[0]);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        transform: Transform::from_translation(Vec3::new(
            position[0].x,
            position[0].y,
            0.0
        )),
        ..Default::default()
    }).insert(Player { 
        speed: 500.0,
    });
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) ||
            keyboard_input.pressed(KeyCode::A) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) ||
            keyboard_input.pressed(KeyCode::D) {
            x_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) ||
            keyboard_input.pressed(KeyCode::W) {
            y_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) ||
            keyboard_input.pressed(KeyCode::S) {
            y_direction -= 1.0;
        }

        let translation = &mut transform.translation;
        translation.x += time.delta_seconds() * x_direction * player.speed;
        translation.x = translation.x.min(380.0).max(-380.0);
        translation.y += time.delta_seconds() * y_direction * player.speed;
        translation.y = translation.y.min(380.0).max(-380.0);
    }
}
