use bevy::prelude::*;

// mod main_menu;
// use main_menu::MainMenuPlugin;

mod window;
use window::WinPlugin;

mod game;
use game::BoardPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((WinPlugin, BoardPlugin))
        .run()
}

