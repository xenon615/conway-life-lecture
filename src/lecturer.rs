use bevy::{
    prelude::*,
    scene::SceneInstanceReady,
};


use crate:: {
    common::*,
    camera::Cam, 
    GameState,
    NotReady
};

use bevy_gltf_animator_helper::{AllAnimations, AniData};

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
pub struct LecturerPath(pub Vec<Vec3>);

//  ==================================================================================================================

pub struct LecturerPlugin;
impl Plugin for LecturerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<LecturerState>()
        .add_systems(OnEnter(GameState::Intro), enter_intro)
        .add_systems(Startup, load)
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
    mut cmd: Commands,
    asset: ResMut<AssetServer>,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    all_animations.add("Lecturer", "models/lecturer.glb", 3, &mut graphs, &asset);
    cmd.spawn((
        SceneRoot(asset.load(GltfAssetLabel::Scene(0).from_asset("models/lecturer.glb"))),
        Transform::from_xyz(0., 0., 50.),
        Lecturer,
        NotReady,
        PathIndex(0),
        AniData::new("Lecturer", 2)
    ))
    .observe(on_ready)
    ;
    cmd.insert_resource(LecturerPath(
        vec![
            Vec3::new(0., 0., 25.), 
            Vec3::new(11., 0., 25.),
            Vec3::new(11., 0., 8.),
            Vec3::new(0., 0., 6.), 
            Vec3::new(0., 1., 11.), 
            Vec3::new(0., 1., 18.)
        ]
    ));
}

// ---

fn on_ready(
    tr: Trigger<SceneInstanceReady>,
    mut cmd: Commands,
) {
    cmd.entity(tr.target()).remove::<NotReady>();
}

// ---

fn enter_intro(
    q: Single<&mut AniData, With<Lecturer>>,
    mut next: ResMut<NextState<LecturerState>>
) {
    q.into_inner().animation_index = 0;
    next.set(LecturerState::Walk);
}

// ---

fn camera_follow (
    q_l: Single<&Transform , With<Lecturer>>,
    q_c: Single<&mut Transform, (With<Cam>, Without<Lecturer>)>,
    time: Res<Time>
) {
    let target =  q_l.into_inner();
    let mut cam = q_c.into_inner();
    let bias = Vec3::new(0., 3., 15.);
    let desired = target.translation + target.rotation.mul_vec3(bias);
    cam.translation = cam.translation.lerp(desired, time.delta_secs() * 10.);
    let look_at = target.translation + target.rotation.mul_vec3(Vec3::Y * 1.5);
    cam.rotation = cam.rotation.slerp(cam.looking_at(look_at, Vec3::Y).rotation, time.delta_secs() * 10.5);
}

// ---

fn do_walk(
    path: Res<LecturerPath>, 
    q_l: Single<(&mut Transform, &mut PathIndex) , With<Lecturer>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<LecturerState>>
) {
    let (mut t, mut pi) = q_l.into_inner();
    let p = path.0[pi.0];
    let step = time.delta_secs() * 3.;
    let sqr_distance = (p - t.translation).length_squared();

    if sqr_distance > step * step {
        t.rotation = t.rotation.slerp(t.looking_at(p, Vec3::Y).rotation, time.delta_secs() * 5.);
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

// ---

fn on_enter_speak(
    q: Single<&mut AniData, With<Lecturer>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let mut a = q.into_inner();
    a.animation_index = 1;
    next_state.set(GameState::Setup);
}
