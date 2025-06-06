use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub(super) struct DebugFpsPlugin;

impl Plugin for DebugFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_fps_display)
            .add_systems(Update, update_fps_display);
    }
}

#[derive(Component)]
struct FpsText;

fn setup_fps_display(mut commands: Commands) {
    commands.spawn((
        FpsText,
        Text("FPS: ...".to_string()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn update_fps_display(
    query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
    diagnostics: Res<DiagnosticsStore>,
) {
    let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else {
        return;
    };

    let Some(value) = fps.smoothed() else {
        return;
    };

    for text_entity in &query {
        *writer.text(text_entity, 0) = format!("FPS: {:.1}", value);
    }
}
