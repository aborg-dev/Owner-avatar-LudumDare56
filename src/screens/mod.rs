//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
mod loading;
mod score;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        score::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    Splash,
    // TODO: Revert to splash on release.
    #[default]
    Loading,
    Title,
    Credits,
    Gameplay,
    Score,
}
