use bevy::prelude::*;

use crate::{palette, AppState};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(spawn_start_menu))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_start_menu))
            .add_system(start_game_on_click);
    }
}

#[derive(Component)]
struct StartMenuPanel;

#[derive(Component)]
struct StartGameButton;

fn spawn_start_menu(mut commands: Commands) {
    let root = (
        NodeBundle {
            style: Style {
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(40.0)),
                ..default()
            },
            ..default()
        },
        StartMenuPanel,
    );
    let button_1 = (
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
        builder.spawn(button_1);
    });
}

fn despawn_start_menu(query: Query<Entity, With<StartMenuPanel>>, mut commands: Commands) {
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
