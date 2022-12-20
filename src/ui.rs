use bevy::prelude::*;

use crate::{
    palette,
    structures::NexusSpawnEvent,
    subjects::content::{SubjectBlueprint, ELVEN_ARCHER, ELVEN_FAST_ARCHER, ELVEN_SNIPER_ARCHER},
    Kingdom,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(spawn_on_click);
    }
}

#[derive(Component)]
struct SpawnButton {
    blueprint: SubjectBlueprint,
}

impl SpawnButton {
    pub fn new(blueprint: SubjectBlueprint) -> Self {
        Self { blueprint }
    }
}

fn setup(mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(40.0)),
            // TODO: add gap when bevy upgrades to taffy v0.2
            ..default()
        },
        ..default()
    };
    let button_1 = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                ..default()
            },
            background_color: palette::DARK_BLUE.into(),
            ..default()
        },
        SpawnButton::new(ELVEN_ARCHER),
    );
    let button_2 = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Undefined, Val::Undefined),
                ..default()
            },
            background_color: palette::DARK_YELLOW.into(),
            ..default()
        },
        SpawnButton::new(ELVEN_FAST_ARCHER),
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
        SpawnButton::new(ELVEN_SNIPER_ARCHER),
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(button_1);
        builder.spawn(button_2);
        builder.spawn(button_3);
    });
}

fn spawn_on_click(
    query: Query<(&Interaction, &SpawnButton), Changed<Interaction>>,
    mut events: EventWriter<NexusSpawnEvent>,
) {
    for (interaction, spawn) in &query {
        if matches!(interaction, Interaction::Clicked) {
            let event = NexusSpawnEvent::new(spawn.blueprint.clone(), Kingdom::Elven);
            events.send(event);
        }
    }
}
