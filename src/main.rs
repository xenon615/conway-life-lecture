#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
// use bevy::winit::WinitSettings;
use bevy::window::WindowResolution;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_gltf_animator_helper::AnimatorHelperPlugin;

mod common;
mod camera;
mod life;
mod ui;
mod env;
mod lecturer;
mod people;

#[derive(Component)]
pub struct NotReady;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
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
        AnimatorHelperPlugin,
        ui::UIPlugin,
        // WorldInspectorPlugin::new()
    ))
    .add_systems(Update, check_ready.run_if(in_state(GameState::Loading)))
    .init_state::<GameState>()
    .run();

}    

fn check_ready(
    not_ready_q: Query<&NotReady>,
    mut next: ResMut<NextState<GameState>>     
) {
    if not_ready_q.is_empty() {
        println!("GAME!");
        // next.set(GameState::Intro);
        next.set(GameState::Setup);
    }
}

