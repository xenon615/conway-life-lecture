use std::time::Duration;
// use bevy::core_pipeline::core_2d::graph;
use bevy::prelude::*;
use crate::common::CurrentAnimation;
use crate::lecturer::LecturerAnimations;
use crate::people::PeopleAnimations;

// ---

pub fn switch_animation<T, T1>(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    objects_q: Query<&CurrentAnimation, (Changed<CurrentAnimation>, With<T1>)>,
    animations: Res<T>,
) 
    where T: Resource + GenAnim, T1: Component
{

    for ca in objects_q.iter() {
        if let Ok((mut player, mut transitions)) =  animation_players.get_mut(ca.1){
            transitions
            .play(
                &mut player,
                animations.get(ca.0),
                Duration::from_millis(250),
            )
            .repeat();            
        }
    }
}

// ---

pub fn load_animation<T> (
    mut commands: Commands,
) where T: Resource + GenAnim + FromWorld {
    commands.init_resource::<T>();
}

// ---

pub fn setup_animation<T, T1 >(
    mut commands: Commands,
    animations: Res<T>,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    objects_q: Query<Entity, (With<T1>, Without<CurrentAnimation>)>, 
    children_q : Query<&Children>
) where T: Resource + GenAnim, T1: Component {


    for o_entity in objects_q.iter() {
        for c  in children_q.iter_descendants(o_entity) {
            if let Ok((entity, mut player)) = players.get_mut(c)  {
                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(&mut player, animations.get(animations.get_len() - 1), Duration::ZERO)
                    .repeat();
            
                commands
                    .entity(entity)
                    .insert(animations.graph())
                    .insert(transitions)
                ;
                
                commands.entity(o_entity).insert(CurrentAnimation(0, entity));
        
            }
        }
    }
}


// ---

impl FromWorld for LecturerAnimations {
    fn from_world(
        world: &mut World,
     ) -> Self {
        let mut graph = AnimationGraph::new();
        let animations = graph
        .add_clips(
            (0..3).into_iter().map(|i| {
                world.load_asset(GltfAssetLabel::Animation(i).from_asset("models/lecturer.glb"))
            }),
            1.0,
            graph.root,
        )
        .collect();
        Self{ animations, graph:  world.resource_mut::<Assets<AnimationGraph>>().add(graph)}
    }
}

// ---

impl FromWorld for PeopleAnimations {
    fn from_world(
        world: &mut World,
     ) -> Self {
        let mut graph = AnimationGraph::new();
        let animations = graph
        .add_clips(
            (0..5).into_iter().map(|i| {
                world.load_asset(GltfAssetLabel::Animation(i).from_asset("models/girl.glb"))
            }),
            1.0,
            graph.root,
        )
        .collect();
        Self{ animations, graph:  world.resource_mut::<Assets<AnimationGraph>>().add(graph)}
    }
}

// ---



pub trait GenAnim {
    fn get(&self, idx: usize) -> AnimationNodeIndex;
    fn graph(&self) -> Handle<AnimationGraph>;
    fn get_len(&self) -> usize;
}

impl GenAnim for LecturerAnimations {
    fn get(&self, idx: usize) -> AnimationNodeIndex {
        self.animations[idx]
    }
    fn graph(&self) -> Handle<AnimationGraph> {
        self.graph.clone()
    }
    fn get_len(&self) -> usize {
        self.animations.len()
    }
}

impl GenAnim for PeopleAnimations {
    fn get(&self, idx: usize) -> AnimationNodeIndex {
        self.animations[idx]
    }
    fn graph(&self) -> Handle<AnimationGraph> {
        self.graph.clone()
    }
    fn get_len(&self) -> usize {
        self.animations.len()
    }
}


