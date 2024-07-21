use bevy::prelude::*;
#[derive(Component)]
pub struct CurrentAnimation(pub usize, pub Entity);
#[derive(Component)]
pub struct PathIndex(pub usize);
