use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::health::Health;
use crate::spaceship::Spaceship;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TEXT_FONT_SIZE: f32 = 40.;


#[derive(Component, Default, Debug)]
pub struct GameScore {
    pub value: i32,
}


#[derive(Event, Debug)]
pub struct GameScoreChangeEvent {
    pub score_delta: i32,
    pub clear_score: bool,
}


#[derive(Component, Default, Debug)]
pub struct HudItem;


#[derive(Component, Default, Debug)]
pub struct HpHudItem;


#[derive(Component, Default, Debug)]
pub struct ScoreHudItem;


pub struct HudPlugin;


impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameScoreChangeEvent>()
            .add_systems(Startup, setup_score)
            .add_systems(Update, (
                (update_game_score, update_game_score_in_hud).chain(),
                update_hp_in_hud,
            ).in_set(InGameSet::EntityUpdates))
            .add_systems(OnEnter(GameState::GameOver), clear_game_score)
            .add_systems(OnExit(GameState::Menu), setup_hud);
    }
}


fn setup_score(mut commands: Commands) {
    commands.spawn(GameScore::default());
}


fn setup_hud(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        },
        HudItem,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section("HP: 0", TextStyle {
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            }),
            HpHudItem,
        ));
        parent.spawn((
            TextBundle::from_section("Score: 0", TextStyle {
                font_size: TEXT_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            }),
            ScoreHudItem,
        ));
    });
}


fn update_hp_in_hud(mut query: Query<&mut Text, With<HpHudItem>>, health_query: Query<&Health, With<Spaceship>>) {
    let Ok(health) = health_query.get_single() else { return };
    let Ok(mut hp_hud_item) = query.get_single_mut() else { return };
    if hp_hud_item.sections.is_empty() {
        return;
    }
    hp_hud_item.sections[0].value = format!("HP: {}", health.value as i32);
}


fn update_game_score_in_hud(mut query: Query<&mut Text, With<ScoreHudItem>>, score_query: Query<&GameScore>) {
    let Ok(score) = score_query.get_single() else { return };
    let Ok(mut score_hud_item) = query.get_single_mut() else { return };
    if score_hud_item.sections.is_empty() {
        return;
    }
    score_hud_item.sections[0].value = format!("Score: {}", score.value);
}


fn update_game_score(
    mut score_change_event_reader: EventReader<GameScoreChangeEvent>,
    mut query: Query<&mut GameScore>,
) {
    let Ok(mut score) = query.get_single_mut() else { return };
    for &GameScoreChangeEvent{ score_delta, clear_score } in score_change_event_reader.read() {
        if clear_score {
            score.value = 0;
        }
        score.value += score_delta;
    }
}


fn clear_game_score(mut score_change_event_writer: EventWriter<GameScoreChangeEvent>) {
    score_change_event_writer.send(GameScoreChangeEvent {
        score_delta: 0,
        clear_score: true,
    });
}
