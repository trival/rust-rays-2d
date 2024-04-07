use rays_2d::*;

fn rnd() -> f64 {
	rand::random::<f64>()
}

pub fn main() {
	let width = 600;
	let height = 400;

	let w = width as f64;
	let h = height as f64;

	let mut image = Image::new(width, height);
	let mut objects = vec![];

	for _ in 0..20 {
		let start = vec2(rnd() * w, rnd() * h);
		let end = vec2(rnd() * w, rnd() * h);
		let line = Line::new(start, end);
		let color = vec3(rnd(), rnd(), rnd());
		let is_light = rnd() < 0.33;
		objects.push(SceneObject::new(to_static(line), color, is_light));
	}

	let scene = Scene::new(to_static(objects));

	image.render(&scene, 100, 50);

	print!("{}", image.to_ppm());
}
