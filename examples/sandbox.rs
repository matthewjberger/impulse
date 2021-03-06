use impulse::{Body, ForceRegistration, Gravity, PhysicsWorld};
use kiss3d::{
    camera::ArcBall,
    event::{Action, Key, WindowEvent},
    light::Light,
    window::Window,
};
use na::{Point3, Translation3, UnitQuaternion, Vector3};
use nalgebra as na;

fn main() {
    // Setup scene
    let mut camera = ArcBall::new(Point3::new(20.0, 18.0, 6.0), Point3::origin());
    let mut window = Window::new("Impulse Physics Engine - Sandbox");
    window.set_light(Light::StickToCamera);

    // Add a ground plane for reference
    let mut ground = window.add_quad(10.0, 10.0, 1, 1);
    ground.set_local_translation(Translation3::new(0.0, -2.0, 0.0));
    ground.set_local_rotation(UnitQuaternion::from_axis_angle(
        &Vector3::x_axis(),
        90_f32.to_radians(),
    ));

    let mut physics_world = PhysicsWorld::default();

    // Register forces
    let gravity = physics_world
        .force_generators
        .insert(Box::new(Gravity::default()));

    // Register bodies
    let body_handle = physics_world.bodies.insert(Body {
        inverse_mass: 2.0_f32.recip(),
        damping: 0.99,
        position: impulse::Vector3::new(0.0, 8.0, 0.0),
        ..Default::default()
    });

    // Link forces to all bodies
    physics_world
        .registrations
        .push(ForceRegistration::new(gravity, vec![body_handle]));

    let mut spheres = Vec::new();

    let mut simulation_active = false;

    while window.render_with_camera(&mut camera) {
        for event in window.events().iter() {
            if let WindowEvent::Key(key, Action::Press, _) = event.value {
                if let Key::Space = key {
                    simulation_active = !simulation_active
                }
            }
        }

        // Fake the last frame's duration
        let last_frame_duration = 20.0_f32.recip();

        if simulation_active {
            physics_world.tick(last_frame_duration);
        }

        for (index, (_current_body_handle, body)) in physics_world.bodies.iter().enumerate() {
            let sphere = match spheres.get_mut(index) {
                Some(sphere) => sphere,
                None => {
                    let mut sphere = window.add_sphere(0.5);
                    sphere.set_color(0.0, 1.0, 1.0);
                    spheres.push(sphere);
                    &mut spheres[index]
                }
            };

            sphere.set_local_translation(Translation3::new(
                body.position.x,
                body.position.y,
                body.position.z,
            ));
        }
    }
}
