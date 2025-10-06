use bevy::prelude::*;

use super::styles::NORMAL_BUTTON_COLOR;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
#[require(Button, BackgroundColor(NORMAL_BUTTON_COLOR))]
pub struct PlayButton;

#[derive(Component)]
#[require(Button, BackgroundColor(NORMAL_BUTTON_COLOR))]
pub struct QuitButton;
