#![forbid(unsafe_code)]
#![forbid(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod particle;
pub mod vec;

pub use self::{particle::*, vec::*};

pub type Real = f32;

#[must_use]
pub fn reals_are_equal(a: Real, b: Real) -> bool {
	(a - b).abs() < Real::EPSILON
}

/// # Panics
///
/// Will panic if actual and expected are not equal
/// within `Real::Epsilon`
pub fn assert_equal(actual: Real, expected: Real) {
	assert!(
		reals_are_equal(actual, expected),
		"left: {actual:?} not equal right: {expected:?}",
	);
}
