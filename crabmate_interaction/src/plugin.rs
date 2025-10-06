use bevy::prelude::*;

use crate::component::ChessInteraction;

pub struct ChessInteractionPlugin;

impl Plugin for ChessInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interactions);
    }
}

fn handle_interactions(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut sprite_query: Query<(&GlobalTransform, &Sprite, &mut ChessInteraction)>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    for (transform, sprite, mut interaction) in sprite_query.iter_mut() {
        let sprite_pos = transform.translation().truncate();
        let sprite_size = sprite.custom_size.unwrap_or(Vec2::new(32.0, 32.0));
        let half_size = sprite_size / 2.0;

        let min = sprite_pos - half_size;
        let max = sprite_pos + half_size;

        if world_pos.x >= min.x
            && world_pos.x <= max.x
            && world_pos.y >= min.y
            && world_pos.y <= max.y
        {
            if buttons.just_pressed(MouseButton::Left) {
                *interaction = ChessInteraction::Pressed;
            } else {
                *interaction = ChessInteraction::Hovered;
            }
        } else {
            *interaction = ChessInteraction::None;
        }
    }
}
