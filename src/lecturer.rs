use bevy::prelude::*;
use crate::common::*;
use crate::animations::{load_animation, switch_animation, setup_animation};
use crate::{camera::Cam, GameState};

#[derive(Component)]
pub struct Lecturer;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LecturerState {
    #[default]
    Load,
    Walk,
    Speak
}

#[derive(Resource)]
pub struct LecturerAnimations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Resource)]
pub struct LecturerPath(pub Vec<Vec3>);

//  ==================================================================================================================

pub struct LecturerPlugin;
impl Plugin for LecturerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<LecturerState>()
        .add_systems(Startup, (load, load_animation::<LecturerAnimations>))
        .add_systems(Update, (setup_animation::<LecturerAnimations, Lecturer>, setup).chain().run_if(in_state(LecturerState::Load)))
        .add_systems(Update, switch_animation::<LecturerAnimations, Lecturer>.run_if(not(in_state(LecturerState::Load))))
        .add_systems(Update, (
            do_walk, 
            camera_follow
        ).run_if(in_state(LecturerState::Walk)))
        .add_systems(OnEnter(LecturerState::Speak), on_enter_speak)
        ;
    }
}

// ---

fn load(
    mut commands: Commands,
    asset: ResMut<AssetServer>,
) {
  
    commands.spawn((
        SceneBundle{
            scene: asset.load("models/lecturer.glb#Scene0"),
            transform: Transform::from_xyz(0., 0., 50.),    
            ..default()
        },
        Lecturer,
        PathIndex(0)
    ));
    
    commands.insert_resource(LecturerPath(
        vec![
            Vec3::new(0., 0., 25.), 
            Vec3::new(11., 0., 25.),
            Vec3::new(11., 0., 8.),
            Vec3::new(0., 0., 6.), 
            Vec3::new(0., 1., 11.), 
            Vec3::new(0., 1., 18.)
        ]
    ))
}

// ---

fn setup(
    mut next_state: ResMut<NextState<LecturerState>>,
    objects_q: Query<Entity, (With<Lecturer>, Without<CurrentAnimation>)>, 
) {
    if objects_q.is_empty() {
        next_state.set(LecturerState::Walk);
        return;
    }
}

// ---

fn camera_follow (
    q_l: Query<&Transform , With<Lecturer>>,
    mut q_c: Query<&mut Transform, (With<Cam>, Without<Lecturer>)>,
    time: Res<Time>
) {

    if let Ok(target) = q_l.get_single() {
        if let Ok(mut cam) = q_c.get_single_mut() {
            let bias = Vec3::new(0., 3., -5.);
            let desired = target.translation + target.right() * bias.x + target.up() * bias.y + target.forward() * bias.z;
            cam.translation = cam.translation.lerp(desired, time.delta_seconds() * 1.);
            cam.rotation = cam.rotation.slerp(cam.looking_at(target.translation, Vec3::Y).rotation, time.delta_seconds() * 5.);
        }
    }
}

// ---

fn do_walk(
    path: Res<LecturerPath>, 
    mut q_l: Query<(&mut Transform, &mut PathIndex) , With<Lecturer>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<LecturerState>>
) {
    if let Ok((mut t, mut pi)) = q_l.get_single_mut() {
        let p = path.0[pi.0];
        let step = time.delta_seconds() * 3.;
        let sqr_distance = (p - t.translation).length_squared();

        if sqr_distance > step * step {
            t.rotation = t.rotation.slerp(t.looking_at(p, Vec3::Y).rotation, time.delta_seconds() * 5.);
            let m = t.forward() * step;
            t.translation += m;
        } else {
            if pi.0 < (path.0.len() - 1) {
                pi.0 += 1;
            } else {
                next_state.set(LecturerState::Speak)
            }
        }
    }
}

// ---

fn on_enter_speak(
    mut q: Query<&mut CurrentAnimation, With<Lecturer>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let mut a = q.get_single_mut().unwrap();
    a.0 = 1;
    next_state.set(GameState::Setup);
}
