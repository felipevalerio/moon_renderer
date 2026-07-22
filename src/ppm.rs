use crate::Framebuffer;

pub fn create_ppm(framebuffer: &mut Framebuffer) {

	for y in 0..framebuffer.height {
		for x in 0..framebuffer.width {
			
			let r = (255.999 * (x as f64 / (framebuffer.width - 1) as f64)) as u8;
			let g = (255.999 * (y as f64 / (framebuffer.height - 1) as f64)) as u8;
			let b = 0;

			framebuffer.set_pixels(x, y, r, g, b);
		}
	}
}
