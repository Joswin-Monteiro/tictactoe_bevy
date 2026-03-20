use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, board_setup);
    }
}

fn board_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: percent(100),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::px(3, 100.0),
                    grid_template_rows: RepeatedGridTrack::px(3, 100.0),
                    margin: UiRect::all(Val::Px(15.0)),
                    ..default()
                })
                .with_children(|parent| {
                    for _ in 1..=9 {
                        parent.spawn((
                            Node {
                                width: Val::Px(33.3),
                                height: Val::Px(33.3),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(1.0, 0.0, 0.0)),
                            BackgroundColor(Color::srgb(0.0, 0.0, 0.8)),
                        ));
                    }
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    border: UiRect::all(Val::Px(2.0)),
                    height: Val::Px(30.0),
                    ..default()
                })
                .with_children(|parent| {
                    for _ in 1..=2 {
                        parent.spawn((
                            Node {
                                width: Val::Px(33.3),
                                height: Val::Px(33.3),
                                border: UiRect::all(Val::Px(2.0)),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(1.0, 0.0, 0.0)),
                            BackgroundColor(Color::srgb(1.0, 0.0, 0.8)),
                        ));
                    }
                });
        });
}
