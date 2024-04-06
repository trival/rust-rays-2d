use crate::geometry::*;
use crate::math::*;

pub struct SceneObject {
	pub geometry: &'static dyn Hit,
	pub color: Vec3,
	pub is_light: bool,
}

impl SceneObject {
	pub fn new(geometry: &'static dyn Hit, color: Vec3, is_light: bool) -> Self {
		SceneObject {
			geometry,
			color,
			is_light,
		}
	}
}

pub struct Scene {
	objects: &'static [SceneObject],
}

impl Scene {
	pub fn new(objects: &'static [SceneObject]) -> Self {
		Scene { objects }
	}
}

const LIGHT_DISTANCE: f64 = 100.0;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
	a * (1.0 - t) + b * t
}

pub fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
	if depth == 0 {
		return Vec3::ZERO;
	}

	let mut closest_hit: Option<HitData> = None;
	let mut closest_object: Option<&SceneObject> = None;

	for object in scene.objects.iter() {
		if let Some(hit) = object.geometry.hit(ray, 0.001, f64::INFINITY) {
			if let Some(current_hit) = closest_hit {
				if hit.t < current_hit.t {
					closest_hit = Some(hit);
					closest_object = Some(object);
				}
			} else {
				closest_hit = Some(hit);
				closest_object = Some(object);
			}
		}
	}

	if let Some(hit) = closest_hit {
		let object = closest_object.unwrap();

		if object.is_light {
			return object.color;
		}
		let target = hit.point + hit.normal + Vec2::random_in_unit_sphere();
		let new_ray = Ray::new(hit.point, target - hit.point);

		return object.color
			* ray_color(&new_ray, scene, depth - 1)
			* lerp(0., 1., (LIGHT_DISTANCE - hit.t) / LIGHT_DISTANCE).clamp(0., 1.);
	}

	Vec3::ZERO
}

pub struct Image {
	pub width: usize,
	pub height: usize,
	pub data: Vec<Vec3>,
}

impl Image {
	pub fn new(width: usize, height: usize) -> Self {
		Image {
			width,
			height,
			data: vec![Vec3::ZERO; (width * height) as usize],
		}
	}

	fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
		self.data[y * self.width + x] = color;
	}

	pub fn render(&mut self, scene: &Scene, samples: u32, max_depth: u32) {
		let s = samples as f64;
		let s = vec3(s, s, s);
		for y in 0..self.height {
			for x in 0..self.width {
				let mut color = Vec3::ZERO;
				for _ in 0..samples {
					let u = (x as f64 + rand::random::<f64>()) as f64;
					let v = (y as f64 + rand::random::<f64>()) as f64;

					let ray = Ray::new(vec2(u, v), Vec2::random_in_unit_sphere());

					color += ray_color(&ray, scene, max_depth) / s;
				}
				self.set_pixel(x, y, color);
			}
		}
	}

	pub fn to_ppm(&self) -> String {
		let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

		for color in self.data.iter() {
			let r = (255.0 * color.x) as u32;
			let g = (255.0 * color.y) as u32;
			let b = (255.0 * color.z) as u32;

			ppm.push_str(&format!("{} {} {}\n", r, g, b));
		}

		ppm
	}
}