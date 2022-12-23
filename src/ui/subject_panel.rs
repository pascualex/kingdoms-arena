use bevy::prelude::*;

use crate::{
    palette,
    structure::NexusSpawnEvent,
    subject::content::{SubjectBlueprint, ELVEN_ARCHER, ELVEN_FAST_ARCHER, ELVEN_SNIPER_ARCHER},
    AppState, Kingdom,
};

pub struct SubjectPanelPlugin;

impl Plugin for SubjectPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_subject_panel))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_subject_panel))
            .add_system(spawn_subject_on_click);
    }
}

#[derive(Component)]
struct SubjectPanel;

#[derive(Component)]
struct SubjectButton {
    blueprint: &'static SubjectBlueprint,
}

impl SubjectButton {
    pub fn new(blueprint: &'static SubjectBlueprint) -> Self {
        Self { blueprint }
    }
}

fn spawn_subject_panel(mut commands: Commands) {
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
                // TODO: add gap when bevy upgrades to taffy v0.2
                ..default()
            },
            ..default()
        },
        SubjectPanel,
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
        SubjectButton::new(&ELVEN_ARCHER),
    );
    let button_2 = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                margin: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            background_color: palette::DARK_YELLOW.into(),
            ..default()
        },
        SubjectButton::new(&ELVEN_FAST_ARCHER),
    );
    let button_3 = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                ..default()
            },
            background_color: palette::DARK_ORANGE.into(),
            ..default()
        },
        SubjectButton::new(&ELVEN_SNIPER_ARCHER),
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(button_1);
        builder.spawn(button_2);
        builder.spawn(button_3);
    });
}

fn despawn_subject_panel(query: Query<Entity, With<SubjectPanel>>, mut commands: Commands) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn spawn_subject_on_click(
    query: Query<(&Interaction, &SubjectButton), Changed<Interaction>>,
    mut events: EventWriter<NexusSpawnEvent>,
) {
    for (interaction, spawn) in &query {
        if matches!(interaction, Interaction::Clicked) {
            let event = NexusSpawnEvent::new(spawn.blueprint, Kingdom::Elven);
            events.send(event);
        }
    }
}
