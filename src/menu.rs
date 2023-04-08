use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::Menu)));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MenuBundle;

#[derive(Component)]
pub enum ButtonAction {
    PlayButton,
    ExitButton,
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TextureAssets>,
) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(30.), Val::Px(0.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).insert(MenuBundle)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(400.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(textures.logo.clone()),
                        style: Style {
                            size: Size::width(Val::Px(400.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                            },
                        ..default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.,
                        flex_wrap: FlexWrap::Wrap,
                        padding: UiRect::new(Val::Px(30.), Val::Px(30.), Val::Px(30.), Val::Px(30.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Ahoy matey!!, Ready for an adventure?",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 50.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ).with_text_alignment(TextAlignment::Center).with_style(Style {
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }));
                    parent.spawn(TextBundle::from_section(
                        "Be prepared for action! The enemies are already eager to take you down.",
                        TextStyle {
                            font: font_assets.fira_sans_reg.clone(),
                            font_size: 24.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ).with_text_alignment(TextAlignment::Center).with_style(Style {
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }));
                    parent.spawn(TextBundle::from_section(
                        "How to play you ask? The controls are simple:",
                        TextStyle {
                            font: font_assets.fira_sans_reg.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ).with_text_alignment(TextAlignment::Center).with_style(Style {
                            max_size: Size::width(Val::Px(500.)),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }));
                    parent.spawn(TextBundle::from_section(
                        "Use the arrow keys or A and D keys to steer yer boat. Change direction to navigate and avoid obstacles on the high seas.",
                        TextStyle {
                            font: font_assets.fira_sans_reg.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ).with_text_alignment(TextAlignment::Center).with_style(Style {
                            max_size: Size::width(Val::Px(500.)),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }));
                    parent.spawn(TextBundle::from_section(
                        "Set yer sights on enemies with yer trusty mouse. Move it to aim, and fire with the mouse button to blast 'em!",
                        TextStyle {
                            font: font_assets.fira_sans_reg.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ).with_text_alignment(TextAlignment::Center).with_style(Style {
                            max_size: Size::width(Val::Px(500.)),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }));
                    parent.spawn(TextBundle::from_section(
                        "Hit yer Escape button if when ye be needin' a break to pause the game.",
                        TextStyle {
                            font: font_assets.fira_sans_reg.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ).with_text_alignment(TextAlignment::Center).with_style(Style {
                            max_size: Size::width(Val::Px(500.)),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                // size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.)),
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: UiImage::new(textures.btn_play.clone()),
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                    },
                                ..default()
                            });
                        }).insert(ButtonAction::PlayButton);
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.)),
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: UiImage::new(textures.btn_exit.clone()),
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                    },
                                ..default()
                            });
                        }).insert(ButtonAction::ExitButton);
                });
        });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match action {
                ButtonAction::PlayButton => {
                    state.set(GameState::Init);
                },
                ButtonAction::ExitButton => {
                    state.set(GameState::End);
                }
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<MenuBundle>>) {
    commands.entity(menu.single()).despawn_recursive();
}
