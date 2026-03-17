use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, button_system)
           .insert_resource(ClearColor(Color::BLACK));
    }
}

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Root Node
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(20.0), 
        ..default()
    })
    .with_children(|parent| {
        // Title of the game
        parent.spawn((
            Node { ..default() },
            Text::new("TIC TAC TOE"),
            TextFont { font_size: 40.0, ..default() },
        ));

        // Play Button
        parent.spawn((
            Button,
            PlayButton,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.0, 0.8)),
        )).with_children(|p| {
            p.spawn((Text::new("Play"), TextFont { font_size: 30.0, ..default() }));
        });

        // Quit Button
        parent.spawn((
            Button,
            QuitButton,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.0, 0.0)),
        )).with_children(|p| {
            p.spawn((Text::new("Quit"), TextFont { font_size: 30.0, ..default() }));
        });
    });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&QuitButton>,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut color, play_btn, quit_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if play_btn.is_some() {
                    *color = Color::srgb(0.0, 1.0, 0.0).into(); 
                } else if quit_btn.is_some() {
                    *color = Color::srgb(1.0, 0.0, 0.0).into();
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
