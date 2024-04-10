use std::f64::consts::TAU;
use std::thread;

use crate::geometry::*;
use crate::math::*;

pub struct SceneObject {
	pub geometry: &'static dyn Hittable,
	pub color: Vec3,
	pub is_light: bool,
}

impl SceneObject {
	pub fn new(geometry: &'static dyn Hittable, color: Vec3, is_light: bool) -> Self {
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
	pub fn build(objects: &'static [SceneObject]) -> &'static Self {
		to_static(Scene { objects })
	}
}

pub fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> Vec3 {
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
			return object.color * 2.0;
		}
		let normal = if hit.normal.dot(ray.dir) < 0.0 {
			hit.normal
		} else {
			-hit.normal
		};
		let target = hit.point + normal + Vec2::random_in_unit_sphere();
		let new_ray = Ray::new(hit.point, target - hit.point);

		return object.color * ray_color(&new_ray, scene, depth - 1);
	}

	vec3(0.3, 0.3, 0.3)
}

pub struct Image {
	pub width: usize,
	pub height: usize,
	pub data: Vec<Vec3>,
}

impl Image {
	fn new(width: usize, height: usize) -> Self {
		Image {
			width,
			height,
			data: vec![Vec3::ZERO; (width * height) as usize],
		}
	}

	fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
		self.data[y * self.width + x] = color;
	}

	fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
		self.data[y * self.width + x]
	}

	pub fn render(
		scene: &Scene,
		width: usize,
		height: usize,
		samples: usize,
		max_bounces: usize,
	) -> Self {
		let mut image = Image::new(width, height);
		let s = samples as f64;
		let angle = TAU / samples as f64;
		let s = vec3(s, s, s);
		for y in 0..height {
			for x in 0..width {
				let mut color = Vec3::ZERO;
				for i in 0..samples {
					let u = (x as f64 + rand::random::<f64>()) as f64;
					let v = (y as f64 + rand::random::<f64>()) as f64;

					let angle = i as f64 * angle + angle * rand::random::<f64>();
					let dir = vec2(angle.cos(), angle.sin());
					let ray = Ray::new(vec2(u, v), Vec2::X.rotate(dir));

					color += ray_color(&ray, scene, max_bounces) / s;
				}
				image.set_pixel(x, y, color);
			}
		}
		image
	}

	pub fn render_parallel(
		scene: &'static Scene,
		width: usize,
		height: usize,
		samples: usize,
		max_bounces: usize,
		threads: usize,
	) -> Self {
		let mut handles = vec![];

		for _ in 0..threads {
			handles.push(thread::spawn(move || {
				Image::render(scene, width, height, samples / threads, max_bounces)
			}));
		}

		let mut imgs = Vec::with_capacity(threads);

		for handle in handles {
			imgs.push(handle.join().unwrap());
		}

		for x in 0..width {
			for y in 0..height {
				let mut color = Vec3::ZERO;
				for img in &imgs {
					color += img.get_pixel(x, y);
				}
				color /= threads as f64;
				imgs[threads - 1].set_pixel(x, y, color);
			}
		}

		imgs.pop().unwrap()
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
