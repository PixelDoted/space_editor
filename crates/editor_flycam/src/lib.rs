use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct FlycamSystemSet;

// ---- Components ----
#[derive(Component)]
pub struct Flycam {
    pub enabled: bool,
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for Flycam {
    fn default() -> Self {
        Self {
            enabled: true,
            move_speed: 5.0,
            mouse_sensitivity: 0.005,
        }
    }
}

// ---- Systems ----
fn movement(
    time: Res<Time>,

    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&Flycam, &mut Transform)>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut grab_cursor = false;
    if mouse_button.pressed(MouseButton::Right) {
        let mouse_delta = mouse_motion.read().fold(Vec2::ZERO, |a, b| a - b.delta);
        let move_vector = Vec3::new(
            (keys.pressed(KeyCode::KeyD) as i8 - keys.pressed(KeyCode::KeyA) as i8) as f32,
            (keys.pressed(KeyCode::KeyE) as i8 - keys.pressed(KeyCode::KeyQ) as i8) as f32,
            (keys.pressed(KeyCode::KeyW) as i8 - keys.pressed(KeyCode::KeyS) as i8) as f32,
        );

        let speed_multiplyer =
            (keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) as i8 + 1) as f32;

        for (flycam, mut transform) in query.iter_mut() {
            if !flycam.enabled {
                continue;
            }

            grab_cursor = true;

            let (ey, ex, _) = transform.rotation.to_euler(EulerRot::YXZ);
            transform.rotation = Quat::from_euler(
                EulerRot::YXZ,
                mouse_delta.x * flycam.mouse_sensitivity + ey,
                mouse_delta.y * flycam.mouse_sensitivity + ex,
                0.0,
            );

            let movement = transform.right() * move_vector.x
                + transform.up() * move_vector.y
                + transform.forward() * move_vector.z;

            transform.translation +=
                movement * flycam.move_speed * speed_multiplyer * time.delta_seconds();
        }
    }

    let mut window = primary_window.single_mut();
    if grab_cursor && window.cursor.visible {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    } else if !grab_cursor && !window.cursor.visible {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

// ---- Plugin ----
pub struct FlycamPlugin;

impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.in_set(FlycamSystemSet));
    }
}
