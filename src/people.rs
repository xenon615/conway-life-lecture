use std::f32::consts::PI;
use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_gltf_animator_helper::{AllAnimations, AniData};

use crate::NotReady;

#[derive(Component)]
pub struct People;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PeopleState {
    #[default]
    Load,
    Run,
    Idle
}

const PEOPLE_SPEED: f32 = 6.5;
const PEOPLE_COUNT: usize = 50;

#[derive(Component)]
pub struct PeoplePath(Vec<Vec3>);

#[derive(Component)]
pub struct Running;

//  =========================================================================================================

pub struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PeopleState>()
            .add_systems(Startup, load)
            .add_systems(Update, do_run.run_if(in_state(PeopleState::Run)))
        ;
    }
}

// ---

fn load(
    mut cmd : Commands,
    asset: ResMut<AssetServer>,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,

) {
    all_animations.add("People", "models/girl.glb", 3, &mut graphs, &asset);
    cmd.spawn((
        SceneRoot(asset.load(GltfAssetLabel::Scene(0).from_asset("models/girl.glb"))),
        Transform::from_xyz(fastrand::f32() * 50. -25. , 0., 50.),
        People,
        Running,
        AniData::new("People", 0),
        PeoplePath(calc_path(0)),
        Speed((fastrand::f32() + 1.) * PEOPLE_SPEED),
        
    ))
    .observe(on_ready)
    ;    
    cmd.spawn((People, NotReady));
}

// ---

fn on_ready(
    tr: Trigger<SceneInstanceReady>,
    mut cmd : Commands,
    ready_q: Single<Entity,(With<NotReady>, With<People>)>,
    mut next: ResMut<NextState<PeopleState>>
) {
     for i in 1 .. PEOPLE_COUNT {
        cmd.
        entity(tr.target())
        .clone_and_spawn()
        .insert((
            Transform::from_xyz(fastrand::f32() * 50. -25. , 0., 50.),
            Speed((fastrand::f32() + 1.) * PEOPLE_SPEED),
            PeoplePath(calc_path(i)),
            AniData::new("People", 0),
            People,
            Running
        ));
     }
    cmd.entity(ready_q.into_inner()).despawn();
    next.set(PeopleState::Run);
}

// ---

fn calc_path(idx: usize) -> Vec<Vec3> {
    let mut tmp: Vec<Vec3> = Vec::new();
    let row = idx / 10;
    let z = (50 - row * 5) as f32;
    let sign = if idx % 2 == 0 {-1.}  else {1.};

    let x = sign * ((5 + idx - row * 10 ) as f32 + fastrand::f32() + 1.)  ; 
    tmp.push(Vec3::new(0., 0., z - 2.));
    tmp.push(Vec3::new(x, 0., z - 2.));
    tmp.push(Vec3::new(x, 0., z));

    tmp
} 

// ---

fn do_run(
    mut commands : Commands,
    mut people_q: Query<(&mut Transform, &mut PeoplePath, &Speed, Entity, &mut AniData), (With<People>, With<Running>)>,
    mut next_state: ResMut<NextState<PeopleState>>,
    time: Res<Time>,
) {
    
    if people_q.is_empty() {
        next_state.set(PeopleState::Idle);
        return;
    }

    for (mut t, mut p, s, e, mut ca) in people_q.iter_mut() {
        let point = p.0[0];
        let ds = point.distance_squared(t.translation);
        let step = time.delta_secs() * s.0 * if ds < 4.5 {0.2} else {1.};
        if ds > 0.5 {
            t.rotation = t.rotation.slerp(t.looking_at(point, Vec3::Y).rotation, time.delta_secs() * 5.);
            let m = t.forward() * step;
            t.translation += m;
        } else {
            t.translation = point;
            p.0.remove(0);
            if p.0.len() == 0 {
                commands.entity(e).remove::<Running>();
                t.rotate_y(PI);
                ca.animation_index = fastrand::usize(1 .. 3);
            } 
        }
    }

}