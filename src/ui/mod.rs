mod coin_panel;
mod start_menu;
mod subject_panel;

use bevy::prelude::*;

use self::{
    coin_panel::CoinPanelPlugin, start_menu::StartMenuPlugin, subject_panel::SubjectPanelPlugin,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CoinPanelPlugin)
            .add_plugin(StartMenuPlugin)
            .add_plugin(SubjectPanelPlugin)
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
