use bevy::{prelude::*, window::close_on_esc};
use bevy_ecs_ldtk::prelude::*;
// use iyes_loopless::prelude::*;
use crate::prelude::*;

pub const TILE_SIZE: usize = 256;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                title: "Time Game :)".to_string(),
                ..Default::default()
            },
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest());

    App::new()
        .add_plugins(default_plugins)
        .add_plugin(LdtkPlugin)

        .add_plugin(LevelSetupPlugin)
        .add_plugin(TickUpdatePlugin)

        .insert_resource(Ticks(0))
        .insert_resource(Dims {x: 0, y: 0})
        .insert_resource(Grid::new())

        .insert_resource(ClearColor(Color::hex("1E2B39").unwrap()))
        .insert_resource(LevelSelection::Index(1))
        .insert_resource(LdtkSettings {
            level_background: LevelBackground::Nonexistent,
            ..Default::default()
        })
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<PastPlayerBundle>("PastPlayer")
        .register_ldtk_entity::<BoxBundle>("Box")
        .register_ldtk_entity::<TimeMachinePartBundle>("Time_machine_part")

        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
            .with_system(setup)
        )
        .add_system(close_on_esc)
        .add_state(AppState::InGame) // Change later
        .run();
}

pub mod setup;
pub mod entities;
pub mod tick_update_plugin;
pub mod resources;
pub mod time_travel;

pub mod prelude {
    pub use super::{
        setup::*, entities::*, tick_update_plugin::*, resources::*, time_travel::*
    };
}