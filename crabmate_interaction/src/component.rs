use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub enum ChessInteraction {
    Pressed,
    Hovered,
    #[default]
    None,
}
