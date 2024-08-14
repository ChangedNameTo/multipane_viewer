use bevy::prelude::*;
mod button_plugin;
mod pane_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [800., 600.].into(),
                title: "Bevy CSS Grid Layout Example".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(pane_plugin::PanePlugin)
        .add_plugins(button_plugin::ButtonPlugin)
        .run();
}
