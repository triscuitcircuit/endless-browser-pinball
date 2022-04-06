use endless_pinball::{app, Commands, UiCameraBundle, bevy_main};

#[bevy_main]
pub fn main() {
    app();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

}