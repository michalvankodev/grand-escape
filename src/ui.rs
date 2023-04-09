use bevy::prelude::*;

use crate::{
    health::Health,
    loading::{FontAssets, TextureAssets},
    player::Player,
    power_up::PowerUpExhaustTimers,
    score::GameScore,
    GameState,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnEnter(GameState::Init)))
            .add_system(despawn_ui.in_schedule(OnEnter(GameState::Restart)))
            .add_system(update_score.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_power_ups.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_health_bar.in_set(OnUpdate(GameState::Playing)));
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

#[derive(Component)]
struct PowerUpWrapper;

fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
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
            // Power ups
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    ..default()
                })
                .insert(PowerUpWrapper);
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
                                    Val::Px(10.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                    Val::Px(30.),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Score",
                                TextStyle {
                                    font: font_assets.fira_mono.clone(),
                                    font_size: 22.0,
                                    color: Color::rgb(0.1, 0.1, 0.1),
                                },
                            ));
                            parent
                                .spawn(TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: font_assets.fira_mono.clone(),
                                        font_size: 24.0,
                                        color: Color::rgb(0.1, 0.1, 0.1),
                                    },
                                ))
                                .insert(ScoreText);
                            parent.spawn(TextBundle::from_section(
                                "Distance",
                                TextStyle {
                                    font: font_assets.fira_mono.clone(),
                                    font_size: 18.0,
                                    color: Color::rgb(0.1, 0.1, 0.1),
                                },
                            ));
                            parent
                                .spawn(TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: font_assets.fira_mono.clone(),
                                        font_size: 20.0,
                                        color: Color::rgb(0.1, 0.1, 0.1),
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
                            parent.spawn(TextBundle::from_section(
                                "Time",
                                TextStyle {
                                    font: font_assets.fira_mono.clone(),
                                    font_size: 18.0,
                                    color: Color::rgb(0.1, 0.1, 0.1),
                                },
                            ));
                            parent
                                .spawn(TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: font_assets.fira_mono.clone(),
                                        font_size: 20.0,
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
    time_text.sections[0].value = format!("{}:{:02}", minutes, seconds);

    let mut st = text_q.p1();
    let score_text = &mut st.get_single_mut().unwrap();
    score_text.sections[0].value = format!(
        "{}",
        game_score.score + game_score.distance_traveled as i32 / 50
    );

    let mut dt = text_q.p2();
    let distance_text = &mut dt.get_single_mut().unwrap();
    distance_text.sections[0].value = format!("{:.0}m", game_score.distance_traveled);
}

fn update_health_bar(
    health_q: Query<&Health, With<Player>>,
    mut health_bar_q: Query<&mut Style, With<HealthBar>>,
) {
    let health = health_q.get_single().unwrap();
    let mut health_bar = health_bar_q.get_single_mut().unwrap();

    health_bar.size = Size::width(Val::Percent(
        100. * (health.health_amount as f32 / health.max_health as f32),
    ));
}

fn update_power_ups(
    mut commands: Commands,
    power_up_wrapper_q: Query<(Entity, Option<&Children>), With<PowerUpWrapper>>,
    timers: Res<PowerUpExhaustTimers>,
    textures: Res<TextureAssets>,
) {
    let (wrapper, children) = power_up_wrapper_q.get_single().unwrap();
    let children_len = if let Some(children_s) = children {
        children_s.len()
    } else {
        0
    };
    if timers.weapon.len() == children_len {
        return;
    } else if timers.weapon.len() > children_len {
        commands
            .spawn(ImageBundle {
                image: UiImage::new(textures.power_up_weapon.clone()),
                style: Style {
                    size: Size::width(Val::Px(40.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .set_parent(wrapper);
    } else {
        let children = children.unwrap()[0];
        commands.entity(children).despawn_recursive();
    }
}

fn despawn_ui(mut commands: Commands, ui_q: Query<Entity, With<UiWrapper>>) {
    let ui_entity = ui_q.get_single().unwrap();
    commands.entity(ui_entity).despawn_recursive();
}
