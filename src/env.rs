use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Cathedra;

pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_light, init_env));
        app.register_type::<Cathedra>();
    }
}

// ---

fn init_light(
    mut al : ResMut<AmbientLight>
) {
    al.brightness = 100.;
} 

// ---

fn init_env(
    mut commands: Commands,
    assets: ResMut<AssetServer>
) {
    commands.spawn((
        SceneBundle {
            scene: assets.load("models/scene.glb#Scene0"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(1., 1., 1.)),
            ..default()
        },
    ));

}