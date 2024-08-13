mod hello_world;
mod people;
mod name;

pub(super) struct HelloPlugin;

use bevy::prelude::*;
use hello_world::hello_world;
use people::{add_people, greet_people, update_people};


impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
