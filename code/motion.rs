use crate::*;

const GIZMOS_Z_INDEX: f32 = 30.0;
const REMOVE_DESTINATION_THRESHOLD_RATIO: f32 = 0.1; // of entity speed

#[derive(Resource, Default)]
pub struct ControlledEntity(pub Option<Entity>);

pub struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlledEntity>().add_systems(
            Update,
            (
                update_destination,
                update_position.after(update_destination),
                draw_gizmos.after(update_position), // 📜 run if edit mod or smth
            ),
        );
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub destination: Option<Vec2>,
    pub velocity_desire: Vec2,
    pub velocity: Vec2,
    pub inertia_ratio: f32,
}
impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 100.0,
            destination: None,
            velocity_desire: Vec2::default(),
            velocity: Vec2::default(),
            inertia_ratio: 0.1, // velocity -> velocity_desire
        }
    }
}

fn update_destination(
    gamepad_state: Res<gamepad::GamepadState>,
    controlled_entity: Res<ControlledEntity>,
    mut query: Query<(&Transform, &mut Movement)>,
) {
    if let Some(entity) = controlled_entity.0 {
        if let Ok((transform, mut movement)) = query.get_mut(entity) {
            // 📜 only gamepad, add more
            if gamepad_state.left_stick_deadzone_exceed {
                let change_in_direction = Vec2::new(
                    gamepad_state.left_stick_x,
                    gamepad_state.left_stick_y,
                ) * movement.speed;
                movement.destination = Some(Vec2::new(
                    transform.translation.x + change_in_direction.x,
                    transform.translation.y + change_in_direction.y,
                ))
            } else {
                movement.destination = None
            }
        }
    }
}

pub fn update_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Movement)>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut transform, mut movement) in query.iter_mut() {
        movement.velocity_desire =
            calculate_velocity_desire(&transform, &mut movement);
        movement.velocity = calculate_velocity(&mut movement, delta_seconds);
        let remove_destination_threshold =
            movement.speed * REMOVE_DESTINATION_THRESHOLD_RATIO;
        transform.translation.x += movement.velocity.x * delta_seconds;
        transform.translation.y += movement.velocity.y * delta_seconds;
        if movement.velocity.length() < remove_destination_threshold {
            movement.destination = None
        }
    }
}

fn calculate_velocity_desire(
    transform: &Transform,
    movement: &mut Movement,
) -> Vec2 {
    if let Some(destination) = movement.destination {
        let current_position =
            Vec2::new(transform.translation.x, transform.translation.y);
        (destination - current_position).clamp_length_max(movement.speed)
    } else {
        Vec2::ZERO
    }
}

fn calculate_velocity(movement: &mut Movement, delta_seconds: f32) -> Vec2 {
    let delta_inertia = delta_seconds / movement.inertia_ratio;
    // 📜 unpack a bit :)
    movement.velocity * (1.0 - delta_inertia)
        + movement.velocity_desire * delta_inertia
}

fn draw_gizmos(
    controlled_entity: Res<ControlledEntity>,
    query: Query<(&Transform, &Movement)>,
    mut gizmos: Gizmos,
) {
    if let Some(entity_id) = controlled_entity.0 {
        if let Ok((transform, movement)) = query.get(entity_id) {
            let velocity_desire = Vec3::new(
                transform.translation.x + movement.velocity_desire.x,
                transform.translation.y + movement.velocity_desire.y,
                GIZMOS_Z_INDEX,
            );
            gizmos.line(transform.translation, velocity_desire, Color::WHITE);
            let velocity = Vec3::new(
                transform.translation.x + movement.velocity.x,
                transform.translation.y + movement.velocity.y,
                GIZMOS_Z_INDEX,
            );
            gizmos.line(transform.translation, velocity, Color::WHITE);
            if let Some(destination) = &movement.destination {
                gizmos.circle_2d(*destination, 1.0, Color::WHITE);
            }
        }
    }
}
