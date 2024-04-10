pub mod geometry;
pub mod math;
pub mod render;

pub use geometry::*;
pub use math::*;
pub use render::*;

pub fn to_static<T>(t: T) -> &'static T {
	Box::leak(Box::new(t))
}

pub fn rnd() -> f64 {
	rand::random::<f64>()
}
