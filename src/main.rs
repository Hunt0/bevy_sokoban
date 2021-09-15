mod player;
mod map;

use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.3, 0.5)))
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(map::MapPlugin)
        // .add_plugin(HelloPlugin)
        .run();
}

// // Plugins
// pub struct HelloPlugin;
//
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//             .add_startup_system(add_people.system())
//             .add_system(greet_people.system());
//     }
// }
//
// // Entities
// struct Person;
//
// // Components
// struct Name(String);
// struct GreetTimer(Timer);
//
// // Systems
// fn add_people(mut commands: Commands) {
//     commands.spawn().insert(Person).insert(Name("Person One".to_string()));
//     commands.spawn().insert(Person).insert(Name("Person Two".to_string()));
//     commands.spawn().insert(Person).insert(Name("Person Three".to_string()));
// }
//
// fn greet_people(
//     time: Res<Time>,
//     mut timer: ResMut<GreetTimer>,
//     query: Query<&Name, With<Person>>
// ) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("Hello {}!", name.0);
//         }
//     }
// }
//
