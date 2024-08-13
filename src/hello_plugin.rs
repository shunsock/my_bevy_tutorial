mod people;
mod name;
mod greet_timer;

pub(super) struct HelloPlugin;

use bevy::prelude::*;
use people::{add_people, greet_people, update_people};
use greet_timer::GreetTimer;


impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
