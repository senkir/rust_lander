use bevy::prelude::*;


#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct LoadingText;

pub fn spawn_loading_screen(mut commands: Commands) {
    commands
        .spawn((
            LoadingScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            parent.spawn((
                LoadingText,
                Text::new("Loading..."),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::BLACK), //TODO modofied from white
            ));
        });

    info!("Loading screen spawned");
}

pub fn animate_loading(time: Res<Time>, mut query: Query<&mut Text, With<LoadingText>>) {
    for mut text in query.iter_mut() {
        //this wraps the pointer in a special type
        let dots = (time.elapsed_secs() * 2.0) as usize % 4;
        //double dereference for text
        **text = format!("Loading{}", ".".repeat(dots));
    }
}

pub fn despawn_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("Loading screen despawned");
}
