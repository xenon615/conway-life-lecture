use bevy::prelude::*;
pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (init_light, init_env))
        ;
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
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("models/scene2.glb"))),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(1., 1., 1.))
    ));

}