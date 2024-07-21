use std::f32::consts::PI;
use bevy::prelude::*;
use crate::common::CurrentAnimation;
use crate::animations::{load_animation, switch_animation, setup_animation};

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

#[derive(Resource)]
pub struct PeopleAnimations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
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
            .add_systems(Startup, (load, load_animation::<PeopleAnimations>))
            .add_systems(Update, (setup_animation::<PeopleAnimations, People>, setup).chain().run_if(in_state(PeopleState::Load)))
            .add_systems(Update, (do_run, switch_animation::<PeopleAnimations, People>).run_if(in_state(PeopleState::Run)))
        ;
    }
}

// ---

fn load(
    mut commands : Commands,
    asset: ResMut<AssetServer>,
) {
   
    let ph = asset.load("models/girl.glb#Scene0");

    for i in 0 .. PEOPLE_COUNT {
        commands.spawn((
            SceneBundle {
                scene: ph.clone(),
                transform: Transform::from_xyz(fastrand::f32() * 50. -25. , 0., 50.),
                ..default()
            },
            People,
            Speed((fastrand::f32() + 1.) * PEOPLE_SPEED),
            PeoplePath(calc_path(i)),
            Running
        ));
    }
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

fn setup(
    mut next_state: ResMut<NextState<PeopleState>>,
    objects_q: Query<Entity, (With<People>, Without<CurrentAnimation>)>, 
) {
    if objects_q.is_empty() {
        next_state.set(PeopleState::Run);
        return;
    }
}

// ---

fn do_run(
    mut commands : Commands,
    mut people_q: Query<(&mut Transform, &mut PeoplePath, &Speed, Entity, &mut CurrentAnimation), (With<People>, With<Running>)>,
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
        let step = time.delta_seconds() * s.0 * if ds < 3.5 {0.2} else {1.};
        if ds > 0.5 {
            t.rotation = t.rotation.slerp(t.looking_at(point, Vec3::Y).rotation, time.delta_seconds() * 5.);
            let m = t.forward() * step;
            t.translation += m;
        } else {
            t.translation = point;
            p.0.remove(0);
            if p.0.len() == 0 {
                commands.entity(e).remove::<Running>();
                t.rotate_y(PI);
                ca.0 = fastrand::usize(1 .. 3);
            } 
        }
    }

}