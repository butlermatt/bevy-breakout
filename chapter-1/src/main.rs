use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_resource(WindowDescriptor{
            title: "Bevy Breakout!".to_string(),
            ..Default::default()
        })
        .add_default_plugins()
        .run();
}
