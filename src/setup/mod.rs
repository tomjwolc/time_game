pub use super::*;

pub mod level_setup_plugin;
pub use level_setup_plugin::*;

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let camera = Camera2dBundle::default();
    let ldtk_handle = LdtkWorldBundle {
        ldtk_handle: asset_server.load("test_level.ldtk"),
        ..Default::default()
    };

    commands.spawn(camera);
    commands.spawn(ldtk_handle);
}