use crate::math::*;

pub struct Ray {
	origin: Vec2,
	direction: Vec2,
}

impl Ray {
	pub fn new(origin: Vec2, direction: Vec2) -> Self {
		Ray {
			origin,
			direction: direction.normalize(),
		}
	}

	pub fn at(&self, t: f64) -> Vec2 {
		self.origin + self.direction * t
	}
}

#[derive(Clone, Copy)]
pub struct HitData {
	pub t: f64,
	pub point: Vec2,
	pub normal: Vec2,
}

pub trait Hit {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData>;
}

pub struct Line {
	start: Vec2,
	end: Vec2,
}

impl Line {
	pub fn new(start: Vec2, end: Vec2) -> Self {
		Line { start, end }
	}

	pub fn translate(&self, offset: Vec2) -> Self {
		Line {
			start: self.start + offset,
			end: self.end + offset,
		}
	}

	pub fn rotate(&self, angle: f64, origin: Vec2) -> Self {
		let rot = vec2(angle.cos(), angle.sin());
		let start = (self.start - origin).rotate(rot) + origin;
		let end = (self.end - origin).rotate(rot) + origin;
		Line { start, end }
	}
}

impl Hit for Line {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData> {
		let p1 = self.start;
		let p2 = self.end;
		let p3 = ray.origin;
		let p4 = ray.origin + ray.direction;
		let denominator = (p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y);
		if denominator == 0.0 {
			return None;
		}
		let u_a = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / denominator;
		let u_b = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / denominator;
		if u_a >= 0.0 && u_a <= 1.0 && u_b >= 0.0 && u_b <= 1.0 {
			let x = p1.x + u_a * (p2.x - p1.x);
			let y = p1.y + u_a * (p2.y - p1.y);

			let point = Vec2::new(x, y);
			let t = (point - ray.origin).length();

			if (t < min_t) || (t > max_t) {
				return None;
			}

			let normal = (p2 - p1).normalize().perp();
			Some(HitData { t, point, normal })
		} else {
			None
		}
	}
}
