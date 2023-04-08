use bevy::prelude::*;

use crate::{
    loading::{FontAssets, TextureAssets},
    score::GameScore,
    GameState,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_score.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
struct UiWrapper;

#[derive(Component)]
struct DistanceText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct HealthBar;

fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>, textures: Res<TextureAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                ..default()
            },
            ..default()
        })
        .insert(UiWrapper)
        .with_children(|parent| {
            // Score board
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::new(
                                    Val::Px(30.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(
                                    "Score: ",
                                    TextStyle {
                                        font: font_assets.fira_mono.clone(),
                                        font_size: 28.0,
                                        color: Color::rgb(0.1, 0.1, 0.1),
                                    },
                                ))
                                .insert(ScoreText);
                            parent
                                .spawn(TextBundle::from_section(
                                    "Distance: ",
                                    TextStyle {
                                        font: font_assets.fira_mono.clone(),
                                        font_size: 28.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ))
                                .insert(DistanceText);
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                flex_grow: 5.,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::new(
                                    Val::Px(30.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(300.), Val::Px(30.)),
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    background_color: Color::rgba(0.3, 0.3, 0.3, 0.7).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(80.),
                                                    Val::Percent(100.),
                                                ),
                                                flex_direction: FlexDirection::Row,
                                                ..default()
                                            },
                                            background_color: Color::rgb(0.92, 0.1, 0.1).into(),
                                            ..default()
                                        })
                                        .insert(HealthBar);
                                });
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                padding: UiRect::new(
                                    Val::Px(30.),
                                    Val::Px(10.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(
                                    "Time: ",
                                    TextStyle {
                                        font: font_assets.fira_mono.clone(),
                                        font_size: 24.0,
                                        color: Color::rgb(0.1, 0.1, 0.1),
                                    },
                                ))
                                .insert(TimeText);
                        });
                });
        });
}

fn update_score(
    game_score: Res<GameScore>,
    mut text_q: ParamSet<(
        Query<&mut Text, With<TimeText>>,
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<DistanceText>>,
    )>,
) {
    let mut tt = text_q.p0();
    let time_text = &mut tt.get_single_mut().unwrap();
    let minutes = game_score.elapsed_time.elapsed().as_secs() / 60;
    let seconds = game_score.elapsed_time.elapsed().as_secs() % 60;
    time_text.sections[0].value = format!("Time: {}:{:02}", minutes, seconds);

    let mut st = text_q.p1();
    let score_text = &mut st.get_single_mut().unwrap();
    score_text.sections[0].value = format!("Score: {}", game_score.score);

    let mut dt = text_q.p2();
    let distance_text = &mut dt.get_single_mut().unwrap();
    distance_text.sections[0].value = format!("Distance: {:.0}m", game_score.distance_traveled);
}
