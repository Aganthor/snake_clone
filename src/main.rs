use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7,0.7, 0.7);

#[derive(Component)]
struct SnakeHead;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0,10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}