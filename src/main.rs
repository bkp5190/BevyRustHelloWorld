use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Boney Patel".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);