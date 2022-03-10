use bevy::{prelude::*};
use bevy_kira_audio::{Audio, AudioPlugin, InstanceHandle, PlaybackState};

use crate::{MenuButtonAction, SelectedOption};
use crate::states::{game, GameTimer, HOVERED_BUTTON, HOVERED_PRESSED_BUTTON, MenuState, NORMAL_BUTTON, OnGameScreen, PRESSED_BUTTON};
use crate::utils::sound::{LoopAudioInstanceHandle, stop_music};


use super::{despawn_screen, DisplayQuality, GameState, TEXT_COLOR, Volume};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(game)
                    .with_system(button_system)
                    .with_system(pause_action)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_screen::<OnGameScreen>)
                    .with_system(stop_music)
                ,
            );
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    volume: Res<crate::Volume>,
) {
    audio.set_volume(volume.0 as f32);
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 4.0 })),
        material: materials.add(Color::rgb(0.41, 0.05, 0.67).into()),
        transform: Transform::from_xyz(0.0, 0.04, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..Default::default()
    });
    // let asset_handle= asset_server.load("music/Disco.ogg");
    // let instance_handle = audio.play_looped(asset_handle);
    // commands.insert_resource(LoopAudioInstanceHandle{instance_handle});
    let button_style = Style {
        size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: TEXT_COLOR,
    };
    commands
        // First create a `NodeBundle` for centering what we want to display
        .spawn_bundle(NodeBundle {
            style: Style {
                // This will center the current node
                margin: Rect::all(Val::Percent(1.0)),
                size: Size{height: Val::Percent(8.0), width: Val::Percent(8.0)},
                // This will display its children in a column, from top to bottom. Unlike
                // in Flexbox, Bevy origin is on bottom left, so the vertical axis is reversed
                flex_direction: FlexDirection::ColumnReverse,
                // `align_items` will align children on the cross axis. Here the main axis is
                // vertical (column), so the cross axis is horizontal. This will center the
                // children
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(OnGameScreen)
        .with_children(|parent| {
            parent.spawn_bundle(ButtonBundle{
                style: button_style,
                color: NORMAL_BUTTON.into(),
                ..Default::default()
            })
                .insert(MenuButtonAction::BackToMainMenu)
                .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section("||", button_text_style, Default::default()),
                    ..Default::default()
                });
            });
        });

    // let music = asset_server.load("music/Disco.ogg");
    // audio.play(music);

    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, false)));
}


// Tick the timer, and change state when finished
pub fn game(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu).unwrap();
    }

}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in interaction_query.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn  pause_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<crate::GameState>>,
    audio: Res<Audio>,
    loop_audio: Res<LoopAudioInstanceHandle>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            let state = audio.state(loop_audio.instance_handle.clone());
            match state{
                PlaybackState::Paused {..} => audio.resume(),
                _ => audio.pause()
            }
            println!("Loop audio {:?}", state);

        }
    }
}