use crate::*;

const GIZMOS_Z_INDEX: f32 = 30.0;
const REMOVE_DESTINATION_THRESHOLD_RATIO: f32 = 0.1; // of entity speed

#[derive(Resource, Default)]
pub struct ControlledEntity(pub Option<Entity>);

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlledEntity>().add_systems(
            Update,
            (
                update_destination,
                update_position.after(update_destination),
                draw_gizmos.after(update_position),
            ),
        );
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub destination: Vec2,
    pub velocity_desire: Vec2,
    pub velocity: Vec2,
    pub inertia: f32,
}
impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 80.0,
            destination: Vec2::ZERO,
            velocity_desire: Vec2::default(),
            velocity: Vec2::default(),
            inertia: 0.2, // in seconds
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
            if gamepad_state.ls_deadzone_exceed {
                let change_in_direction =
                    Vec2::new(gamepad_state.ls_x, gamepad_state.ls_y) * movement.speed;
                movement.destination = Vec2::new(
                    transform.translation.x + change_in_direction.x,
                    transform.translation.y + change_in_direction.y,
                )
            } else {
                movement.destination = Vec2::ZERO;
            }
        }
    }
}

pub fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    let delta_seconds = time.delta_seconds();
    for (mut transform, mut movement) in query.iter_mut() {
        movement.velocity_desire = if movement.destination != Vec2::ZERO {
            let current_position = Vec2::new(transform.translation.x, transform.translation.y);
            (movement.destination - current_position).clamp_length_max(movement.speed)
        } else {
            Vec2::ZERO
        };
        let delta_inertia = delta_seconds / movement.inertia;
        movement.velocity =
            movement.velocity * (1.0 - delta_inertia) + movement.velocity_desire * delta_inertia;
        let remove_destination_threshold = movement.speed * REMOVE_DESTINATION_THRESHOLD_RATIO;
        transform.translation.x += movement.velocity.x * delta_seconds;
        transform.translation.y += movement.velocity.y * delta_seconds;
        if movement.velocity.length() < remove_destination_threshold {
            movement.destination = Vec2::ZERO;
        }
    }
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
            if movement.destination != Vec2::ZERO {
                gizmos.circle_2d(movement.destination, 1.0, Color::WHITE);
            }
        }
    }
}
