use bevy::prelude::*;

use crate::{palette, structures::SpawnerEvent};

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
            ..default()
        },
        ..default()
    };
    let button = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(80.0)),
            ..default()
        },
        background_color: palette::DARK_RED.into(),
        ..default()
    };
    commands.spawn(root).with_children(|builder| {
        builder.spawn(button);
    });
}

fn spawn_on_click(
    query: Query<&Interaction, Changed<Interaction>>,
    mut events: EventWriter<SpawnerEvent>,
) {
    for interaction in &query {
        if matches!(interaction, Interaction::Clicked) {
            events.send(SpawnerEvent);
        }
    }
}
