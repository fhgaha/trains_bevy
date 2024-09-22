mod fps;
mod snowflake;
mod trains_plugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            trains_plugin::TrainsPlugin,
            fps::FpsPlugin,
            // snowflake::SnowflakePlugin,
        ))
        .run();
}
