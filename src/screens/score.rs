//! The title screen that appears when game ends.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

use super::GameScore;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Score), spawn_score_screen);
}

fn spawn_score_screen(mut commands: Commands, game_score: Res<GameScore>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Score))
        .with_children(|children| {
            children.label(format!("Waves cleared: {}", game_score.score));

            children.button("Restart").observe(enter_gameplay_screen);
            children.button("Menu").observe(enter_title_screen);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
