mod coin_panel;
mod recruitment_panel;
mod start_menu;

use bevy::prelude::*;

use self::{
    coin_panel::CoinPanelPlugin, recruitment_panel::RecruitmentPanelPlugin,
    start_menu::StartMenuPlugin,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CoinPanelPlugin)
            .add_plugin(RecruitmentPanelPlugin)
            .add_plugin(StartMenuPlugin)
            .init_resource::<UiAssets>();
    }
}

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.resource();
        UiAssets {
            font: asset_server.load("fonts/roboto_bold.ttf"),
        }
    }
}
