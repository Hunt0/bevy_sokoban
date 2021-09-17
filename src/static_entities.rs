use bevy::prelude::*;
use crate::map::Map;
use crate::constants;

pub struct StaticEntitiesPlugin;

impl Plugin for StaticEntitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_walls.system().after("load_map"));
    }
}

struct Wall {}
struct Floor {}
struct Box {}
struct Spot {}

pub fn create_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let material = materials.add(asset_server.load("images/wall.png").into());
    let wall_positions = map.entity_positions.get(&'W').unwrap();

    for position in wall_positions {
        println!("POSITION {}", position);
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands.spawn_bundle(SpriteBundle {
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(
                    position.x * constants::OFFSET,
                    position.y * constants::OFFSET,
                    0.0
            )),
            ..Default::default()
        }).insert(Wall { 
        });
    }
}
