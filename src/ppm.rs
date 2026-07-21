use crate::Framebuffer;

pub fn create_ppm(framebuffer: &mut Framebuffer) {

	for j in 0..framebuffer.height {
		for i in 0..framebuffer.width {
			let r: f64 = i as f64 / (framebuffer.width - 1) as f64;
			let g: f64 = j as f64 / (framebuffer.height - 1) as f64;
			let b: f64 = 0.0;

			let ir: i32 = (255.999 * r) as i32;
			let ig: i32 = (255.999 * g) as i32;
			let ib: i32 = (255.999 * b) as i32;

			println!("{} {} {}", ir, ig, ib);

			framebuffer.frame_vec.push(ir as u8);
			framebuffer.frame_vec.push(ig as u8);
			framebuffer.frame_vec.push(ib as u8);
		}
	}
}