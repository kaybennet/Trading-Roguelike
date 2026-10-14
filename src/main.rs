use bevy::prelude::*;

#[derive(Resource, Default)]
struct Score(u64);

#[derive(Component)]
struct ScoreText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Update, (click_to_score, update_text))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Text2d::new("0"), ScoreText));
}

fn click_to_score(buttons: Res<ButtonInput<MouseButton>>, mut score: ResMut<Score>) {
    if buttons.just_pressed(MouseButton::Left) {
        score.0 += 1;
    }
}

fn update_text(score: Res<Score>, mut q: Query<&mut Text2d, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut q {
            text.0 = score.0.to_string();
        }
    }
}
