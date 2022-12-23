use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    palette,
    recruitment::RecruitmentEvent,
    subject::content::{SubjectBlueprint, ELVEN_ARCHER, ELVEN_FAST_ARCHER, ELVEN_SNIPER_ARCHER},
    AppState, Kingdom,
};

use super::UiAssets;

pub struct RecruitmentPanelPlugin;

impl Plugin for RecruitmentPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn))
            .add_system(recruit_on_click);
    }
}

#[derive(Component)]
struct RecruitmentPanel;

#[derive(Component)]
struct RecruitmentButton {
    blueprint: &'static SubjectBlueprint,
}

impl RecruitmentButton {
    fn new(blueprint: &'static SubjectBlueprint) -> Self {
        Self { blueprint }
    }
}

fn spawn(assets: Res<UiAssets>, mut commands: Commands) {
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
        RecruitmentPanel,
    );
    let space = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(20.0), Val::Px(20.0)),
            ..default()
        },
        ..default()
    };
    commands.spawn(root).with_children(|builder| {
        recruitment_button(
            builder.spawn_empty(),
            &ELVEN_ARCHER,
            palette::DARK_BLUE,
            &assets,
        );
        builder.spawn(space.clone());
        recruitment_button(
            builder.spawn_empty(),
            &ELVEN_FAST_ARCHER,
            palette::DARK_YELLOW,
            &assets,
        );
        builder.spawn(space.clone());
        recruitment_button(
            builder.spawn_empty(),
            &ELVEN_SNIPER_ARCHER,
            palette::DARK_ORANGE,
            &assets,
        );
    });
}

fn despawn(query: Query<Entity, With<RecruitmentPanel>>, mut commands: Commands) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn recruit_on_click(
    query: Query<(&Interaction, &RecruitmentButton), Changed<Interaction>>,
    mut events: EventWriter<RecruitmentEvent>,
) {
    for (interaction, spawn) in &query {
        if matches!(interaction, Interaction::Clicked) {
            events.send(RecruitmentEvent::new(spawn.blueprint, Kingdom::Elven));
        }
    }
}

fn recruitment_button(
    mut commands: EntityCommands,
    blueprint: &'static SubjectBlueprint,
    color: Color,
    assets: &UiAssets,
) {
    let root = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        },
        RecruitmentButton::new(blueprint),
    );
    let text = TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                blueprint.value.to_string(),
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 20.0,
                    color: palette::DARK_BLACK,
                },
            )],
            alignment: TextAlignment::TOP_RIGHT,
        },
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    };
    commands.insert(root).with_children(|builder| {
        builder.spawn(text);
    });
}
