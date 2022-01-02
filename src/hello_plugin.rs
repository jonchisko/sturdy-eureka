use bevy::core::{Time, Timer};
use bevy::prelude::{Commands, IntoSystem, Plugin, Query, Res, ResMut, With};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

struct Person;

struct Name(String);

struct GreetTimer(Timer);

fn add_people(mut cmds: Commands) {
    cmds.spawn()
        .insert(Person)
        .insert(Name("Krin Mijevski".to_string()));
    cmds.spawn()
        .insert(Person)
        .insert(Name("Siri Sirovich".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for person in query.iter() {
            println!("I greet you traveler, my name is {}!", person.0);
        }
    }
}
