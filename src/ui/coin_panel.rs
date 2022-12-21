use bevy::prelude::*;

use crate::{palette, ui::UiAssets, AppState, Coins};

pub struct CoinPanelPlugin;

impl Plugin for CoinPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_coin_panel))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_coin_panel))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_coin_text));
    }
}

#[derive(Component)]
struct CoinPanel;

#[derive(Component)]
struct CoinText;

fn spawn_coin_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(40.0), Val::Px(40.0), Val::Auto),
                ..default()
            },
            ..default()
        },
        CoinPanel,
    );
    let coin_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::DARK_BLACK,
                },
            ),
            ..default()
        },
        CoinText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(coin_text);
    });
}

fn despawn_coin_panel(query: Query<Entity, With<CoinPanel>>, mut commands: Commands) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn update_coin_text(coins: Res<Coins>, mut query: Query<&mut Text, With<CoinText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", coins.elven);
}
