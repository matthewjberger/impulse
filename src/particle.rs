use crate::{vector::Vector3, Real};

#[derive(Debug, Default)]
pub struct Particle {
    /// Holds the linear position of the particle in world space
    pub position: Vector3,

    /// Holds the linear velocity of the particle in world space
    pub velocity: Vector3,

    /// Holds the acceleration of the particle.
    /// This value can be used to set the acceleration
    /// due to gravity (its primary use) or any other constant acceleration.
    pub acceleration: Vector3,

    /// Holds the amount of damping applied to linear
    /// motion. Damping is required to remove energy added
    /// through numerical instability in the integrator.
    pub damping: Real,

    /// Holds the inverse of the mass of the body.
    ///
    /// It is more useful to hold the inverse mass because
    /// integration is simpler, and because in real-time
    /// simulation it is more useful to have objects with
    /// infinite mass (immovable) than zero mass
    /// (completely unstable in numerical simulation).
    pub inverse_mass: Real,

    /// Holds the accumulated force to be applied at the next
    /// simulation iteration only. This value is zeroed at each
    /// integration step.
    pub force_accumulator: Vector3,
}

impl Particle {
    pub fn mass(&self) -> Real {
        self.inverse_mass.recip()
    }

    pub fn has_infinite_mass(&self) -> bool {
        self.inverse_mass == 0.0
    }

    pub fn add_force(&mut self, force: &Vector3) {
        self.force_accumulator += *force;
    }

    /// Integrates the particle forward in time by the given amount.
    /// This function uses a Newton-Euler integration method, which is a
    /// linear approximation to the correct integral. For this reason it
    /// may be inaccurate in some cases.
    fn integrate(&mut self, duration: Real) {
        // Infinite mass should not be integrated
        if self.inverse_mass <= 0.0 && duration > 0.0 {
            return;
        }

        // Update linear position
        self.position += self.velocity * duration;

        // Update linear velocity from the acceleration
        self.velocity += self.acceleration * duration;

        // Impose drag
        self.velocity *= self.damping.powf(duration);

        // Clear any accumulated forces
        self.force_accumulator = Vector3::zero();
    }
}