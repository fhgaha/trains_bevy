mod examples;
mod fps;
mod trains_plugin;

use bevy::prelude::*;
use bevy_mod_raycast::cursor::CursorRayPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            // .set(bevy_mod_raycast::low_latency_window_plugin()) //why the hell this unlocks max fps?
            ,
            trains_plugin::TrainsPlugin,
            fps::FpsPlugin,
            // examples::ecs_example_1::EcsExample1
            // examples::snowflake::SnowflakePlugin
        ))
        .run();
}
