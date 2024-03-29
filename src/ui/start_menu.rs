use bevy::prelude::*;

use crate::{palette, AppState};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(spawn))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn))
            .add_system(start_game_on_click);
    }
}

#[derive(Component)]
struct StartMenu;

#[derive(Component)]
struct StartGameButton;

fn spawn(mut commands: Commands) {
    let root = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(40.0),
                    ..default()
                },
                size: Size {
                    width: Val::Percent(100.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        StartMenu,
    );
    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                ..default()
            },
            background_color: palette::DARK_BLUE.into(),
            ..default()
        },
        StartGameButton,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(button);
    });
}

fn despawn(query: Query<Entity, With<StartMenu>>, mut commands: Commands) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn start_game_on_click(
    query: Query<&Interaction, (With<StartGameButton>, Changed<Interaction>)>,
    mut state: ResMut<State<AppState>>,
) {
    for interaction in &query {
        if matches!(interaction, Interaction::Clicked) {
            state.set(AppState::Game).unwrap();
        }
    }
}
