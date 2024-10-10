use crate::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use iyes_perf_ui::entries::diagnostics::{PerfUiEntryFPS, PerfUiEntryFrameTime};
use iyes_perf_ui::ui::root::PerfUiRoot;
use iyes_perf_ui::PerfUiPlugin;

const ANIMATION_FPS: f32 = 6.0;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameTimeDiagnosticsPlugin, PerfUiPlugin::default()))
            .add_systems(
                Startup,
                (spawn_lira.after(camera::setup_cameras), set_diagnostics),
            )
            .add_systems(Update, animate_sprite);
    }
}

fn set_diagnostics(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryFrameTime::default(),
    ));
}

fn spawn_lira(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut camera_resource: ResMut<camera::CameraResource>,
    mut controlled_entity: ResMut<movement::ControlledEntity>,
) {
    let texture = asset_server.load("lira/idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(75, 127), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 0, last: 7 };

    let lira = commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(0., 0., 100.),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(
                1.0 / ANIMATION_FPS,
                TimerMode::Repeating,
            )),
            movement::Movement::default(),
            camera::PIXEL_LAYER,
        ))
        .id();

    camera_resource.followed_entity = Some(lira);
    controlled_entity.0 = Some(lira);
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
