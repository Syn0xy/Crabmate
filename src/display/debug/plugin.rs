use bevy::prelude::*;

use super::fps_plugin::DebugFpsPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugFpsPlugin);
    }
}
