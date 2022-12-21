mod start_menu;
mod subject_panel;

use bevy::prelude::*;

use self::{start_menu::StartMenuPlugin, subject_panel::SubjectPanelPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StartMenuPlugin)
            .add_plugin(SubjectPanelPlugin);
    }
}
