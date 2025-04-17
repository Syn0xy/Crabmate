use bevy::prelude::*;
use crabmate_chess::ChessPlugin;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChessPlugin)
            .add_systems(Startup, setup_application);
    }
}

fn setup_application(mut commands: Commands) {
    commands.spawn(Camera2d);
}
