mod state;
mod plugins;

use tracing_subscriber::EnvFilter;

use crate::plugins::ui::UiPlugin;
use crate::state::GameState;
use bevy::prelude::*;
use plugins::core::CorePlugin;

fn main() {
    #[cfg(debug_assertions)]
    {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("a_bevy_of_us=debug"));

        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    }
    #[cfg(not(debug_assertions))]
    {
        let file = std::fs::File::create("game.log").expect("Failed to create log file");
        let env_filter = EnvFilter::new("a_bevy_of_us=info");

        tracing_subscriber::fmt()
            .with_writer(file)
            .with_env_filter(env_filter)
            .init();
    }

    App::new()
        .add_plugins((DefaultPlugins, CorePlugin, UiPlugin))
        .init_state::<GameState>()
        .insert_state(GameState::MainMenu)
        .run();
}
