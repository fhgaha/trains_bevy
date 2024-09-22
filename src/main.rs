mod fps;
mod trains_plugin;
mod examples;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            trains_plugin::TrainsPlugin,
            fps::FpsPlugin,
            // examples::ecs_example_1::EcsExample1
            // examples::snowflake::SnowflakePlugin
        ))
        .run();
}
