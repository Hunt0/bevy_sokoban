use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MapStore {
                maps: Vec::new(),
            })
            .insert_resource(Map {
                current_map_index: 0,
                layout: HashMap::new(),
                height: 0,
                width: 0,
            })
            .add_startup_system(load_maps.system().label("load_maps"))
            .add_startup_system(setup_map.system().after("load_maps"));
    }
}

struct MapStore {
    maps: Vec<String>
}

pub struct Map {
    current_map_index: u8,
    layout: HashMap<(u8, u8), String>,
    height: u8,
    width: u8,
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Index: {}\nLayout: {:?}\nHeight: {}\nWidth: {}\n",
            self.current_map_index,
            self.layout,
            self.height,
            self.width,
        )
    }
}

fn load_maps(
    mut commands: Commands,
    mut map_store: ResMut<MapStore>
) {
    println!("Loading Maps");
}

fn setup_map(
    mut commands: Commands,
    mut current_map: ResMut<Map>
) {
    current_map.height = 100;
    current_map.width = 200;
    println!("{}", *current_map);
}
