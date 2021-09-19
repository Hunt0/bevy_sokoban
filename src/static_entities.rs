use bevy::prelude::*;
use crate::map::Map;
use crate::map::Position;
use crate::constants;

pub struct StaticEntitiesPlugin;

impl Plugin for StaticEntitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
                "add_materials",
                SystemStage::single(add_materials.system())
            )
            .add_startup_stage(
                "create_walls",
                SystemStage::single(create_walls.system())
            );

            //.add_startup_system(add_materials.system().after("load_map"))
    }
}

pub struct Wall {
    pub position: Position,
}

struct Floor {}
struct Box {}
struct Spot {}

struct Materials {
    wall_material: Handle<ColorMaterial>,
    floor_material: Handle<ColorMaterial>,
    box_material: Handle<ColorMaterial>,
    spot_material: Handle<ColorMaterial>,
}

fn add_materials(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("ADDING MATERIALS");
    commands.insert_resource(Materials {
        wall_material: materials.add(asset_server.load("images/wall.png").into()),
        floor_material: materials.add(asset_server.load("images/floor.png").into()),
        box_material: materials.add(asset_server.load("images/box.png").into()),
        spot_material: materials.add(asset_server.load("images/spot.png").into()),
    });
}

fn create_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    map: Res<Map>,
) {
    println!("CREATING WALLS");
    let wall_positions = map.entity_positions.get(&'W').unwrap();
    // let wall_material = materials.add(asset_server.load("images/wall.png").into());
    // println!("MATERIALS {:?}", materials);

    for position in wall_positions {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands.spawn_bundle(SpriteBundle {
            material: materials.wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                    position.x * constants::OFFSET,
                    position.y * constants::OFFSET,
                    0.0
            )),
            ..Default::default()
        }).insert(Wall { 
            position: Position {
                x: position.x,
                y: position.y,
            }
        });
    }
}
