use bevy::prelude::*;

use crate::{
    loading::FontAssets,
    menu::{ButtonAction, ButtonColors},
    score::GameScore,
    GameState,
};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_end_ui.in_schedule(OnEnter(GameState::End)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::End)));
    }
}

#[derive(Component)]
pub struct EndMenuBundle;

fn setup_end_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    game_score: Res<GameScore>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(EndMenuBundle)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.), Val::Px(300.)),
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.4, 0.4, 0.4).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Nice try !",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 50.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style {
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            format!(
                                "Final score: {}",
                                game_score.score + game_score.distance_traveled as i32 / 50
                            ),
                            TextStyle {
                                font: font_assets.fira_sans_reg.clone(),
                                font_size: 24.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style {
                            flex_wrap: FlexWrap::Wrap,
                            max_size: Size::width(Val::Px(300.)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            format!("Distance travelled: {:.0}m", game_score.distance_traveled),
                            TextStyle {
                                font: font_assets.fira_sans_reg.clone(),
                                font_size: 24.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style {
                            flex_wrap: FlexWrap::Wrap,
                            max_size: Size::width(Val::Px(300.)),
                            ..default()
                        }),
                    );
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
                                        padding: UiRect::new(
                                            Val::Px(10.),
                                            Val::Px(10.),
                                            Val::Px(10.),
                                            Val::Px(10.),
                                        ),
                                        ..Default::default()
                                    },
                                    background_color: button_colors.normal.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Restart",
                                        TextStyle {
                                            font: font_assets.fira_sans_reg.clone(),
                                            font_size: 24.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                })
                                .insert(ButtonAction::RestartButton);

                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        // size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                                        margin: UiRect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::new(
                                            Val::Px(10.),
                                            Val::Px(10.),
                                            Val::Px(10.),
                                            Val::Px(10.),
                                        ),
                                        ..Default::default()
                                    },
                                    background_color: button_colors.normal.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Exit",
                                        TextStyle {
                                            font: font_assets.fira_sans_reg.clone(),
                                            font_size: 24.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                })
                                .insert(ButtonAction::ExitButton);
                        });
                });
        });
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<EndMenuBundle>>) {
    commands.entity(menu.single()).despawn_recursive();
}
