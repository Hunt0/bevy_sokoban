use bevy::prelude::*;
use crate::map::Map;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_player.system())
            .add_system(player_movement_system.system());
    }
}

struct Player {
    speed: f32,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("images/player.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
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
