use crate::{math::*, to_static};

pub struct Ray {
	pub origin: Vec2,
	pub dir: Vec2,
}

impl Ray {
	pub fn new(origin: Vec2, direction: Vec2) -> Self {
		Ray {
			origin,
			dir: direction.normalize(),
		}
	}

	pub fn at(&self, t: f64) -> Vec2 {
		self.origin + self.dir * t
	}
}

#[derive(Clone, Copy)]
pub struct HitData {
	pub t: f64,
	pub point: Vec2,
	pub normal: Vec2,
}

pub trait Hittable: Send + Sync {
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

pub fn static_line(start: Vec2, end: Vec2) -> &'static Line {
	to_static(Line::new(start, end))
}

impl Hittable for Line {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData> {
		// two line segments run from p to p + r and from q to q + s
		// the intersection point is p + tr = q + us
		// t = (q − p) × s / (r × s)
		// u = (q − p) × r / (r × s)
		//
		// translates to
		// ray from ro[p] to ro[p] + rdir[r]
		// line from pstart[q] to pstart[q] + (pend - pstart)[s]
		// the intersection point is
		// r.orig + t * r.dir = p.start + u * (p.end - p.start)
		// t = (p.start − r.orig) x (p.end - p.start) / (r.dir x (p.end - p.start))
		// u = (p.start − r.orig) x r.dir / (r.dir x (p.end - p.start))
		//

		let s = self.end - self.start;
		let denom = ray.dir.cross(s);
		if (denom - 0.0).abs() < f64::EPSILON {
			return None;
		}

		let q = self.start - ray.origin;

		let u = q.cross(ray.dir) / denom;
		if u < 0.0 || u > 1.0 {
			return None;
		}

		let t = q.cross(s) / denom;
		if t < min_t || t >= max_t {
			return None;
		}

		let p = ray.at(t);
		Some(HitData {
			t,
			point: p,
			normal: s.perp().normalize(),
		})
	}
}
