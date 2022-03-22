use bevy::{input::touch::TouchPhase, prelude::*, window::WindowMode, app::AppExit};

use crate::states::*;

pub struct PausePlugin;

impl Plugin for PausePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_state(PauseState::Disabled)
            .add_system_set(SystemSet::on_enter(GameState::PauseMenu).with_system(pausemenu_setup))

            .add_system_set(SystemSet::on_enter(PauseState::Main).with_system(pause_menu_setup))

            .add_system_set(
                SystemSet::on_exit(PauseState::Main)
                    .with_system(despawn_screen::<OnPauseMenuScreen>),
            )

            // Sound Menu
            // .add_system_set(
            //     SystemSet::on_enter(MenuState::SettingsSound)
            //         .with_system(sound_settings_menu_setup),
            // )
            // .add_system_set(
            //     SystemSet::on_update(MenuState::SettingsSound)
            //         .with_system(setting_button::<Volume>),
            // )
            // .add_system_set(
            //     SystemSet::on_exit(MenuState::SettingsSound)
            //         .with_system(despawn_screen::<OnSoundSettingsMenuScreen>),
            // )
            .add_system_set(
                SystemSet::on_update(GameState::PauseMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
        ;
    }
}

fn pausemenu_setup(mut menu_state: ResMut<State<PauseState>>){
    let _ = menu_state.set(PauseState::Main);
}

fn pause_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positionned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: Rect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(OnPauseMenuScreen)
        .with_children(|parent| {
            // Display the game name


            // Display three buttons for each action available from the main menu:
            // - new game
            // - settings
            // - quit
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(PauseButtonAction::BackToGame)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn_bundle(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Resume",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(PauseButtonAction::Leaderboards)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn_bundle(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Leaderboards",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(OnPauseMenuScreen)
        .with_children(|parent| {
            // Display the game name


            // Display three buttons for each action available from the main menu:
            // - new game
            // - settings
            // - quit
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(PauseButtonAction::SettingsSound)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/wrench.png");
                    parent.spawn_bundle(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Sound",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(PauseButtonAction::BackToMainMenu)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn_bundle(ImageBundle {
                        style: button_icon_style,
                        image: UiImage(icon),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Exit Game", button_text_style, Default::default()),
                        ..Default::default()
                    });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &PauseButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<PauseState>>,
    mut game_state: ResMut<State<crate::GameState>>,
) {
    for (interaction, pause_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match pause_action {
                PauseButtonAction::BackToMainMenu =>{
                    game_state.set(GameState::Menu).unwrap();
                    menu_state.set(PauseState::Disabled).unwrap();
                }
                PauseButtonAction::BackToGame=>{
                    game_state.set(GameState::Game).unwrap();
                    menu_state.set(PauseState::Disabled).unwrap();
                }
                PauseButtonAction::Leaderboards=>{

                }
                PauseButtonAction::SettingsSound =>{
                    menu_state.set(PauseState::SettingsSound).unwrap();
                }
            }
        }
    }
}
