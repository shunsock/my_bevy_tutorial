use bevy::prelude::{Timer, Resource};

#[derive(Resource)]
pub(super) struct GreetTimer(pub(crate) Timer);