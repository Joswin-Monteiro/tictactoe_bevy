use bevy::prelude::*;
use bevy::window::*;

mod main_menu;
use main_menu::MainMenuPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tic-Tac-Toe".to_string(),
                name: Some("Tic-Tac-Toe".to_string()),
                resolution: WindowResolution::new(640, 480), 
                ..default()
            }),
            ..default()
        }),
        MainMenuPlugin))
        .run()
}

