mod application;
mod constants;
mod display;

use bevy::{prelude::*, window::PresentMode};
use constants::application_constants;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: application_constants::WINDOW_TITLE.to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            application::ApplicationPlugin,
            display::DisplayPlugin,
        ))
        .run();
}
