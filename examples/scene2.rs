use std::{fs::File, io::Write};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rays_2d::*;

fn bezier_curve(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f64) -> Vec2 {
	let one_minus_t = 1.0 - t;
	let one_minus_t_squared = one_minus_t * one_minus_t;
	let one_minus_t_cubed = one_minus_t_squared * one_minus_t;
	let t_squared = t * t;
	let t_cubed = t_squared * t;

	one_minus_t_cubed * p0
		+ 3.0 * one_minus_t_squared * t * p1
		+ 3.0 * one_minus_t * t_squared * p2
		+ t_cubed * p3
}

struct Curve {
	p0: Vec2,
	p1: Vec2,
	p2: Vec2,
	p3: Vec2,
}

impl Curve {
	fn new(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
		Curve { p0, p1, p2, p3 }
	}

	fn point(&self, t: f64) -> Vec2 {
		bezier_curve(self.p0, self.p1, self.p2, self.p3, t)
	}
}

fn rnd_point(rnd: f64, start: Vec2, dir: Vec2, offset: f64) -> Vec2 {
	start + dir * (1.0 + rnd * 4.0) + Vec2::random_in_unit_sphere() * offset
}

const WIDTH: usize = 2000;
const HEIGHT: usize = 1500;
// const WIDTH: usize = 800;
// const HEIGHT: usize = 600;
// const WIDTH: usize = 200;
// const HEIGHT: usize = 150;
const SAMPLES_PER_PIXEL: usize = 200;
const MAX_BOUNCES: usize = 50;
const THREADS: usize = 8;

pub fn make_image(seed: u64) -> Image {
	let mut rng = ChaCha8Rng::seed_from_u64(seed);
	let w = WIDTH as f64;
	let h = HEIGHT as f64;

	let center = vec2(w * 0.5, h * 0.4);
	let bottom = vec2(w * 0.5, h * 0.9);
	let left = vec2(0.0, h * 0.5);
	let right = vec2(w, h * 0.5);
	let left_top = vec2(w * 0.2, 0.0);
	let right_top = vec2(w * 0.8, 0.0);

	let left_curve = Curve::new(center, left_top, left, bottom);
	let right_curve = Curve::new(center, right_top, right, bottom);

	let lights_count = 14;
	let lights_count_f = lights_count as f64;
	let rnd_offset = h / 30.;

	let mut objects = vec![];

	for i in 0..(lights_count - 1) {
		let t1 = i as f64 / lights_count_f + rng.gen::<f64>() / lights_count_f;
		let t2 = (i + 1) as f64 / lights_count_f + rng.gen::<f64>() / lights_count_f;

		let l1 = left_curve.point(t1);
		let l2 = left_curve.point(t2);

		let l1d = l1 - l2;
		let pl1 = rnd_point(rng.gen(), l2, l1d, rnd_offset);

		let l2d = l2 - l1;
		let pl2 = rnd_point(rng.gen(), l1, l2d, rnd_offset);

		let color = vec3(rng.gen(), rng.gen(), rng.gen());

		objects.push(SceneObject::new(static_line(pl1, pl2), color, true));

		let t1 = i as f64 / lights_count_f + rng.gen::<f64>() / lights_count_f;
		let t2 = (i + 1) as f64 / lights_count_f + rng.gen::<f64>() / lights_count_f;

		let r1 = right_curve.point(t1);
		let r2 = right_curve.point(t2);

		let r1d = r1 - r2;
		let pr1 = rnd_point(rng.gen(), r2, r1d, rnd_offset);

		let r2d = r2 - r1;
		let pr2 = rnd_point(rng.gen(), r1, r2d, rnd_offset);

		let color = vec3(rng.gen(), rng.gen(), rng.gen());

		objects.push(SceneObject::new(static_line(pr1, pr2), color, true));
	}

	for _ in 0..50 {
		let start = vec2(rng.gen::<f64>() * w * 3. - w, rng.gen::<f64>() * h * 3. - h);
		let end = vec2(rng.gen::<f64>() * w * 3. - w, rng.gen::<f64>() * h * 3. - h);
		let line = static_line(start, end);
		let color = vec3(rng.gen(), rng.gen(), rng.gen());
		objects.push(SceneObject::new(line, color, false));
	}

	let scene = Scene::build(to_static(objects));

	Image::render_parallel(
		&scene,
		WIDTH,
		HEIGHT,
		SAMPLES_PER_PIXEL,
		MAX_BOUNCES,
		THREADS,
		true,
	)
}

fn main() -> std::io::Result<()> {
	// for i in 0..25 {
	// 	let image = make_image(i);
	// 	let file_name = format!("out/foo{}.ppm", i);

	// 	let mut file = File::create(file_name)?;
	// 	file.write_all(image.to_ppm().as_bytes())?;
	// }

	let seed = 18;
	let image = make_image(seed);
	let file_name = format!("out/scene2_seed{}_{}_{}.ppm", seed, WIDTH, HEIGHT);
	let mut file = File::create(file_name)?;
	file.write_all(image.to_ppm().as_bytes())?;

	Ok(())
}
