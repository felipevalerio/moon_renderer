struct Framebuffer {
	width: i32,
	heigth: i32,
	framebuffer: Vec<u8>
}


pub fn create_ppm(width: i32, height: i32) -> Vec<u8> {

	let mut framebuffer: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

	for j in 0..height {
		for i in 0..width {
			let r: f64 = i as f64 / (width - 1) as f64;
			let g: f64 = j as f64 / (height - 1) as f64;
			let b: f64 = 0.0;

			let ir: i32 = (255.999 * r) as i32;
			let ig: i32 = (255.999 * g) as i32;
			let ib: i32 = (255.999 * b) as i32;

			println!("{} {} {}", ir, ig, ib);

			framebuffer.push(ir as u8);
			framebuffer.push(ig as u8);
			framebuffer.push(ib as u8);
		}
	}

	framebuffer
}