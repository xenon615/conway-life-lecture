use bevy::prelude::*;
// use bevy::winit::WinitSettings;
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::*;
use bevy::window::WindowResolution;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod common;
mod camera;
mod life;
mod ui;
mod env;
mod lecturer;
mod people;
mod animations;
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Intro,
    Setup,
    Simulate
}


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    // .insert_resource(WinitSettings::desktop_app())
    .add_plugins((
        DefaultPlugins.set(
            WindowPlugin {
                primary_window : Some(Window {
                    resolution : WindowResolution::new(1400., 900.),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        ),
        camera::CameraPlugin,
        life::FieldPlugin,
        env::EnvPlugin,
        lecturer::LecturerPlugin,
        people::PeoplePlugin,

        ui::UIPlugin,
        
        ExportRegistryPlugin::default(),
        ComponentsFromGltfPlugin{legacy_mode: false}
        // WorldInspectorPlugin::new()
    ))
    .init_state::<GameState>()
    .run();

}    