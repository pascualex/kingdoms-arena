use bevy::prelude::*;

use crate::{
    palette,
    recruitment::RecruitmentEvent,
    subject::content::{SubjectBlueprint, ELVEN_ARCHER, ELVEN_FAST_ARCHER, ELVEN_SNIPER_ARCHER},
    AppState, Kingdom,
};

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
struct SubjectButton {
    blueprint: &'static SubjectBlueprint,
}

impl SubjectButton {
    pub fn new(blueprint: &'static SubjectBlueprint) -> Self {
        Self { blueprint }
    }
}

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
                // TODO: add gap when bevy upgrades to taffy v0.2
                ..default()
            },
            ..default()
        },
        RecruitmentPanel,
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

fn despawn(query: Query<Entity, With<RecruitmentPanel>>, mut commands: Commands) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn recruit_on_click(
    query: Query<(&Interaction, &SubjectButton), Changed<Interaction>>,
    mut events: EventWriter<RecruitmentEvent>,
) {
    for (interaction, spawn) in &query {
        if matches!(interaction, Interaction::Clicked) {
            events.send(RecruitmentEvent::new(spawn.blueprint, Kingdom::Elven));
        }
    }
}
