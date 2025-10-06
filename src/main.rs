mod application;
mod constants;
mod events;
mod ui;

use bevy::{prelude::*, window::PresentMode};
use constants::application_constants;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
}

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
            ui::menu::MainMenuPlugin,
            ui::DebugFpsPlugin,
            application::ApplicationPlugin,
        ))
        .insert_state(AppState::Menu)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 1.0 / 2.0,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
