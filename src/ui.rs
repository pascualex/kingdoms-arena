use bevy::prelude::*;

use crate::{palette, structures::NexusSpawnEvent, subjects::content::ELVEN_ARCHER, Kingdom};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(spawn_on_click);
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
    let button_1 = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(80.0), Val::Px(80.0)),
            ..default()
        },
        background_color: palette::DARK_BLUE.into(),
        ..default()
    };
    let button_2 = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(80.0), Val::Px(80.0)),
            margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Undefined, Val::Undefined),
            ..default()
        },
        background_color: palette::DARK_YELLOW.into(),
        ..default()
    };
    let button_3 = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(80.0), Val::Px(80.0)),
            ..default()
        },
        background_color: palette::DARK_ORANGE.into(),
        ..default()
    };
    commands.spawn(root).with_children(|builder| {
        builder.spawn(button_1);
        builder.spawn(button_2);
        builder.spawn(button_3);
    });
}

fn spawn_on_click(
    query: Query<&Interaction, Changed<Interaction>>,
    mut events: EventWriter<NexusSpawnEvent>,
) {
    for interaction in &query {
        if matches!(interaction, Interaction::Clicked) {
            let event = NexusSpawnEvent::new(ELVEN_ARCHER, Kingdom::Elven);
            events.send(event);
        }
    }
}
