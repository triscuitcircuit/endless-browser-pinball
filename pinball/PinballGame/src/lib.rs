pub mod states;
pub use bevy::window::WindowMode;
pub use states::*;
pub use bevy::prelude::*;
pub use bevy::input::touch::TouchPhase;
pub fn app(){
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(AudioPlugin)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(2))
        .add_startup_system(setup)
        // Declare the game state, and set its startup value
        .add_state(GameState::Menu)
        // Adds the plugins for each state
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(pausemenu::PausePlugin)
        .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

}