use bevy::{input::touch::TouchPhase, prelude::*, window::WindowMode, app::AppExit};
use bevy::audio::AudioSink;

use crate::states::*;
use crate::utils::sound::{LoopAudioInstanceHandle, set_vol, stop_music};

use super::{Volume};
// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_state(MenuState::Disabled)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            // Systems to handle the main menu screen
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
                .add_system_set(
                    SystemSet::on_update(MenuState::Main)
                        .with_system(set_vol)
                )
            .add_system_set(
                SystemSet::on_exit(MenuState::Main)
                    .with_system(despawn_screen::<OnMainMenuScreen>)
                    .with_system(stop_music)
            )
            // Systems to handle the settings menu screen
            .add_system_set(
                SystemSet::on_enter(MenuState::Settings).with_system(settings_menu_setup),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Settings)
                    .with_system(despawn_screen::<OnSettingsMenuScreen>),
            )
            // Systems to handle the display settings screen
            .add_system_set(
                SystemSet::on_enter(MenuState::SettingsDisplay)
                    .with_system(display_settings_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(MenuState::SettingsDisplay)
                    .with_system(setting_button::<DisplayQuality>),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::SettingsDisplay)
                    .with_system(despawn_screen::<OnDisplaySettingsMenuScreen>),
            )
            // Systems to handle the sound settings screen
            .add_system_set(
                SystemSet::on_enter(MenuState::SettingsSound)
                    .with_system(sound_settings_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(MenuState::SettingsSound)
                    .with_system(setting_button::<Volume>),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::SettingsSound)
                    .with_system(despawn_screen::<OnSoundSettingsMenuScreen>),
            )
            // Common systems to all screens that handles buttons behaviour
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system),
            );
    }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn setting_button<T: Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut UiColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in interaction_query.iter() {
        if *interaction == Interaction::Clicked && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn main_menu_setup(mut commands: Commands,
                   asset_server: Res<AssetServer>,
                   audio: Res<Audio>,
                   audio_sinks: Res<Assets<AudioSink>>,
) {
        let asset_handle = asset_server.load("music/CasualArcade.ogg");
        let instance_handle = audio_sinks.get_handle(audio.play_in_loop(asset_handle));
        commands.insert_resource(LoopAudioInstanceHandle(instance_handle));
        let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
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
            font_size: 25.0,
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
            .insert(OnMainMenuScreen)
            .with_children(|parent| {
                // Display the game name
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(50.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Endless Pinball: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: TEXT_COLOR,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });

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
                    .insert(MenuButtonAction::Play)
                    .with_children(|parent| {
                        let icon = asset_server.load("textures/Game Icons/right.png");
                        parent.spawn_bundle(ImageBundle {
                            style: button_icon_style.clone(),
                            image: UiImage(icon),
                            ..Default::default()
                        });
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "New Game",
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
                    .insert(MenuButtonAction::Settings)
                    .with_children(|parent| {
                        let icon = asset_server.load("textures/Game Icons/wrench.png");
                        parent.spawn_bundle(ImageBundle {
                            style: button_icon_style.clone(),
                            image: UiImage(icon),
                            ..Default::default()
                        });
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Settings",
                                button_text_style.clone(),
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            });
    }

fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
        font_size: 25.0,
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
        .insert(OnSettingsMenuScreen)
        .with_children(|parent| {
            // Display two buttons for the submenus
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::SettingsDisplay)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Display",
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
                .insert(MenuButtonAction::SettingsSound)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Sound",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            // Display the back button to return to the main menu screen
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::BackToMainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Back", button_text_style, Default::default()),
                        ..Default::default()
                    });
                });
        });
}

fn display_settings_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_quality: Res<crate::DisplayQuality>,
) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
        font_size: 25.0,
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
        .insert(OnDisplaySettingsMenuScreen)
        .with_children(|parent| {
            // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
            // use the default value, `FlexDirection::Row`, from left to right.
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Display a label for the current setting
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Display Quality",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                    // Display a button for each possible value
                    for quality_setting in [
                        crate::DisplayQuality::Low,
                        crate::DisplayQuality::Medium,
                        crate::DisplayQuality::High,
                    ] {
                        let mut entity = parent.spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                ..button_style.clone()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..Default::default()
                        });
                        entity.insert(quality_setting).with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    format!("{:?}", quality_setting),
                                    button_text_style.clone(),
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                        if *display_quality == quality_setting {
                            entity.insert(SelectedOption);
                        }
                    }
                });
            // Display the back button to return to the settings screen
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::BackToSettings)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Back", button_text_style, Default::default()),
                        ..Default::default()
                    });
                });
        });
}

fn sound_settings_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    volume: Res<crate::Volume>,
) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
        font_size: 25.0,
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
        .insert(OnSoundSettingsMenuScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Volume",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                    for volume_setting in [0,1,2,3,4,5] {
                        let mut entity = parent.spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(30.0), Val::Px(65.0)),
                                ..button_style.clone()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..Default::default()
                        });
                        entity.insert(crate::Volume(volume_setting));
                        if *volume == crate::Volume(volume_setting) {
                            entity.insert(SelectedOption);
                        }
                    }
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(MenuButtonAction::BackToSettings)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Back", button_text_style, Default::default()),
                        ..Default::default()
                    });
                });
        });
}

fn menu_action(
    interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<crate::GameState>>,
    ) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            println!("clicked");
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(crate::GameState::Game).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings).unwrap(),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay).unwrap();
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound).unwrap();
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main).unwrap(),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings).unwrap();
                }
            }
        }
    }
}

