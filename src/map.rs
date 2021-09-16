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
        app.insert_resource(MapStore {
                maps: Vec::new(),
                map_sizes: Vec::new(),
            })
            .insert_resource(Map {
                current_map_index: 0,
                height: 0,
                width: 0,
            })
            .add_startup_system(load_files_into_map_store.system())
            .add_system_set(
                SystemSet::on_enter(AppState::LOADING)
                    .with_system(build_map.system())
            );
    }
}

struct MapStore {
    maps: Vec<HashMap<(usize, usize), char>>,
    map_sizes: Vec<(usize, usize)>,
}

pub struct Map {
    current_map_index: usize,
    height: usize,
    width: usize,
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

fn load_files_into_map_store(
    mut map_store: ResMut<MapStore>
) {
    println!("LOADING MAPS");
    //TODO : fix all the error handling / unwrapping stuff
    let map_paths = read_dir("assets/maps/").unwrap();

    for path in map_paths {
        let file_path = path.unwrap().path();
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open {}: {}", file_path.display(), why),
        };

        let file_lines = BufReader::new(file).lines();
        let mut map_hash: HashMap<(usize, usize), char> = HashMap::new();

        //TODO : fix setting height and width
        let mut height = 0;
        let mut width = 0;

        for (y, file_line) in file_lines.enumerate() {
            if y > height {
                height = y;
            }

            if let Ok(line) = file_line {
                if line.len() > width {
                    width = line.len();
                }

                for (x, character) in line.chars().enumerate() {
                    //TODO : fix unwrap thing
                    map_hash.insert((x, y), character);
                }
            } else {
                panic!("Failed to read line in file");
            }
        }

        map_store.maps.push(map_hash);
        map_store.map_sizes.push((width, height));
    }
}

fn build_map(
    commands: Commands,
    mut current_map: ResMut<Map>
) {
    println!("RUNNING BUILD MAP");
}
