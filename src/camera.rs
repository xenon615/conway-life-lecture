use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseWheel;
use bevy::core_pipeline::Skybox;
use crate::GameState;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup) 
        .add_systems(OnEnter(GameState::Simulate), enter_simulation.run_if(in_state(GameState::Simulate)))
        .add_systems(Update, control.run_if(in_state(GameState::Simulate)))
        .add_systems(OnEnter(GameState::Setup), enter_setup)
        ;
    }
} 

#[derive(Component)]
pub struct Cam;


fn setup (
    mut commands : Commands,
    assets : ResMut<AssetServer>
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 15., 100.),
            projection: PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
                ..default()
            }.into(),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        Skybox{
            image: assets.load("skyboxes/cubemap.ktx2"),
            brightness: 1500.
        }, 
        Cam,
        Name::new("Camera"),
    ));

}

fn control (
    mut q_camera: Query<(&Camera, &GlobalTransform, &mut Transform), With<Cam>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,

) {
    let Ok((camera, camera_gtransform, mut camera_transform)) = q_camera.get_single_mut() else {
        return 
    };

    if buttons.just_pressed(MouseButton::Left) {
        let window = q_window.single();
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let plane_origin = Vec3::ZERO;
        let plane = InfinitePlane3d::new(Vec3::Z);
        let Some(ray) = camera.viewport_to_world(camera_gtransform, cursor_position) else {
            return;
        };
        let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
            return;
        };
        let global_cursor = ray.get_point(distance);
        camera_transform.translation = Vec3::new(global_cursor.x, global_cursor.y, camera_transform.translation.z);
    }
    
    for ev in scroll_evr.read() {
        let m = camera_transform.forward() * ev.y * 5.;
        camera_transform.translation += m;  
    }
}


fn enter_setup (
    mut q_camera: Query<&mut Transform, With<Cam>>
) {
    let Ok(mut camt) = q_camera.get_single_mut() else {
        return;
    };
    camt.translation = Vec3::new(0., 5., 25.);
    camt.look_at(Vec3::new(0., 1., 18.), Vec3::Y);

}

fn enter_simulation (
    mut q_camera: Query<&mut Transform, With<Cam>>
) {
    let Ok(mut camt) = q_camera.get_single_mut() else {
        return;
    };
    camt.translation = Vec3::new(0., 20., 50.);
    camt.look_at(Vec3::new(0.,20., 0.), Vec3::Y);
}