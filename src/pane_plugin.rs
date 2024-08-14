use crate::button_plugin;
use bevy::{color::palettes::css::*, prelude::*};

pub struct PanePlugin;

impl Plugin for PanePlugin {
    fn build(&self, app: &mut App) {
        // Add things to your app here
        app.add_systems(Startup, spawn_layout);
    }
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto-Black.ttf");
    commands.spawn(Camera2dBundle::default());

    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|builder| {
            // Header
            header(builder, &font);

            // Main content grid
            builder
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(24.0)),
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ..default()
                })
                .with_children(|builder| {
                    item_rect(builder, ORANGE);
                    item_rect(builder, BISQUE);
                    item_rect(builder, BLUE);
                    item_rect(builder, CRIMSON);
                    item_rect(builder, AQUA);
                    item_rect(builder, ORANGE_RED);
                    item_rect(builder, DARK_GREEN);
                    item_rect(builder, FUCHSIA);
                    item_rect(builder, TEAL);
                    item_rect(builder, ALICE_BLUE);
                    item_rect(builder, CRIMSON);
                    item_rect(builder, ANTIQUE_WHITE);
                    item_rect(builder, YELLOW);
                    item_rect(builder, DEEP_PINK);
                    item_rect(builder, YELLOW_GREEN);
                    item_rect(builder, SALMON);
                });

            build_modal(builder);
        });
}

fn header(builder: &mut ChildBuilder, font: &Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            button_plugin::create_menu_button(parent, font, "File");
            button_plugin::create_menu_button(parent, font, "Edit");
            button_plugin::create_menu_button(parent, font, "Help");
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take its size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Srgba) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(BLACK.into()),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color.into()),
                ..default()
            });
        });
}

// Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
fn build_modal(builder: &mut ChildBuilder) {
    builder.spawn(NodeBundle {
        visibility: Visibility::Visible,
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect {
                top: Val::Px(100.),
                bottom: Val::Auto,
                left: Val::Auto,
                right: Val::Auto,
            },
            width: Val::Percent(60.),
            height: Val::Px(300.),
            max_width: Val::Px(600.),
            ..default()
        },
        background_color: BackgroundColor(Color::WHITE.with_alpha(0.8)),
        ..default()
    });
}
