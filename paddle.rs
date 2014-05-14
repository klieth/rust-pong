
extern crate sdl;

pub struct Paddle {
	pub x: int,
	pub y: int,
	pub w: int,
	pub h: int
}

impl Paddle {
	pub fn draw(self, ctx: sdl::video::Surface ) {
		ctx.fill_rect(Some(sdl::Rect {
			x: self.x as i16,
			y: self.y as i16,
			w: self.w as u16,
			h: self.h as u16
		}), sdl::video::RGB(0, 255, 0));
	}
}
