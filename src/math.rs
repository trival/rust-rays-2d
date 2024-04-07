pub use glam::f64::{dvec2 as vec2, dvec3 as vec3, DVec2 as Vec2, DVec3 as Vec3};

pub trait Vec2Ext {
	fn random_in_unit_sphere() -> Vec2;
	fn cross(self, other: Self) -> f64;
}

impl Vec2Ext for Vec2 {
	fn random_in_unit_sphere() -> Vec2 {
		loop {
			let p = vec2(
				rand::random::<f64>() * 2.0 - 1.0,
				rand::random::<f64>() * 2.0 - 1.0,
			);

			if p.length_squared() < 1.0 {
				return p;
			}
		}
	}

	fn cross(self, other: Self) -> f64 {
		self.x * other.y - self.y * other.x
	}
}
