use bevy::prelude::*;


fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(greet_world.system())
        .add_system(greet_people.system())
        .run();
}

struct Person;

struct Name(String);

fn add_people(mut cmds: Commands) {
    cmds.spawn().insert(Person).insert(Name("Krin Mijevski".to_string()));
    cmds.spawn().insert(Person).insert(Name("Siri Sirovich".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for person in query.iter() {
        println!("I greet you traveler, my name is {}!", person.0);
    }
}

fn greet_world() {
    println!("Hello World!");
}