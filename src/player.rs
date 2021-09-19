use bevy::prelude::*;
use crate::map::Map;
use crate::map::Position;
use crate::constants;
use crate::static_entities::Wall;
use bevy::sprite::collide_aabb::collide;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_player.system().after("load_map"))
            .add_system(player_movement_system.system());
    }
}

struct Player {
    speed: f32,
    position: Position,
}

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let texture_handle = asset_server.load("images/player.png");
    let position = map.entity_positions.get(&'P').unwrap();

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
        position: Position {
            x: position[0].x,
            y: position[0].y,
        }
    });

    println!("PLAYER POSITION: X {}, Y {}", position[0].x, position[0].y);
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform, &Sprite)>,
    map: Res<Map>
) {
    if let Ok((mut player, mut transform, sprite)) = player_query.single_mut() {
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

        let new_x_position = player.position.x + x_direction;
        let new_y_position = player.position.y + y_direction;
        let new_pos = (new_x_position as i8, new_y_position as i8);

        if map.position_lookup.get(&new_pos).unwrap() != &'W' {
            translation.x += x_direction * player.speed;
            translation.y += y_direction * player.speed;

            player.position.x = player.position.x + x_direction;
            player.position.y = player.position.y + y_direction;
        }
    }
}
