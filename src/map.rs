use std::collections::HashMap;
use std::fmt;
use std::fs::*;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use bevy::prelude::*;
use crate::state::AppState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Map {
                entity_positions: HashMap::new(),
                height: 0.0,
                width: 0.0,
                current_map_index: 0,
            })
            .add_startup_system(load_map.system().label("load_map"));
            // .add_system_set(
            //     SystemSet::on_enter(AppState::LOADING)
            //         .with_system(load_map.system().label("load_map")
            // );
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "X: {}, Y: {}", self.x, self.y)
    }
}

pub struct Map {
    pub entity_positions: HashMap<char, Vec<Position>>,
    pub height: f32,
    pub width: f32,
    pub current_map_index: u8,
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Index: {}\nHeight: {}\nWidth: {}\n",
            self.current_map_index,
            self.height,
            self.width,
        )
    }
}

fn load_map(
    mut map: ResMut<Map>
) {
    let file_string_path = format!("assets/maps/map_{}.txt", map.current_map_index);
    let file_path = Path::new(
        &file_string_path
    );

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(why) => panic!("Failed to open file {}: {}", file_path.display(), why),
    };

    let file_lines = BufReader::new(file).lines();
    //TODO : probably set height & width in a better way

    for (y, file_line) in file_lines.enumerate() {
        let y_float = y as f32;
        if y_float > map.height {
            map.height = y_float;
        }

        if let Ok(line) = file_line {
            let x_float = line.len() as f32;
            if x_float > map.width {
                map.width = x_float;
            }

            for (x, character) in line.chars().enumerate() {
                let position = Position {
                    x: x as f32,
                    y: y as f32,
                };

                if map.entity_positions.contains_key(&character) {
                    map.entity_positions.get_mut(&character).unwrap().push(position);
                } else {
                    let mut position_vec: Vec<Position> = Vec::new();
                    position_vec.push(position);
                    map.entity_positions.insert(character, position_vec);
                }
            }
        } else {
            panic!("Failed to read line in file");
        }
    }
}
