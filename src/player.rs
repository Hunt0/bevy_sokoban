use bevy::prelude::*;
use crate::map::Map;
use crate::constants;

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
            position[0].x * constants::OFFSET,
            position[0].y * constants::OFFSET,
            0.0
        )),
        ..Default::default()
    }).insert(Player { 
        speed: 32.0,
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

        // Movement changed to just_pressed instead of pressed along with 
        // removing the time.delta_seconds() to only move 32 pixels at a time
        
        if keyboard_input.just_pressed(KeyCode::Left) ||
            keyboard_input.just_pressed(KeyCode::A) {
            x_direction -= 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Right) ||
            keyboard_input.just_pressed(KeyCode::D) {
            x_direction += 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Up) ||
            keyboard_input.just_pressed(KeyCode::W) {
            y_direction += 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Down) ||
            keyboard_input.just_pressed(KeyCode::S) {
            y_direction -= 1.0;
        }

        let translation = &mut transform.translation;
        // translation.x += time.delta_seconds() * x_direction * player.speed;
        // translation.y += time.delta_seconds() * y_direction * player.speed;
        translation.x += x_direction * player.speed;
        translation.y += y_direction * player.speed;
    }
}
