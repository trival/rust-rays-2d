use rays_2d::*;

pub fn main() {
	let width = 800;
	let height = 600;
	let samples_per_pixel = 100;
	let max_bounces = 50;
	let threads = 8;

	let w = width as f64;
	let h = height as f64;

	let mut objects = vec![];

	for _ in 0..20 {
		let start = vec2(rnd() * w, rnd() * h);
		let end = vec2(rnd() * w, rnd() * h);
		let line = static_line(start, end);
		let color = vec3(rnd(), rnd(), rnd());
		let is_light = rnd() < 0.33;
		objects.push(SceneObject::new(line, color, is_light));
	}

	let scene = Scene::build(to_static(objects));

	let image = Image::render_parallel(
		&scene,
		width,
		height,
		samples_per_pixel,
		max_bounces,
		threads,
		true,
	);

	print!("{}", image.to_ppm());
}
