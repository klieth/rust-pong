
extern crate sdl;

use paddle::Paddle;
mod paddle;

fn main() {
	sdl::init([sdl::InitVideo]);
	sdl::wm::set_caption("Game Test", "Second parameter");

	let screen = 
		match sdl::video::set_video_mode(800, 600, 32, [sdl::video::HWSurface], [sdl::video::DoubleBuf]) {
			Ok(screen) => screen,
			Err(err) => fail!("Failed to set video mode: {}", err)
		};
	for i in range(0,10) {
		screen.fill_rect(Some(sdl::Rect {
			x: (i as i16) * 800 / 10,
			y: 0,
			w: 800 / 10,
			h: 600
		}), sdl::video::RGB((i as u8) * (255 / 10),0,0) );
	}
	let p = Paddle {x:10, y:10, w:20, h:100};
	println!("{:?}",screen);
	p.draw(screen);
	screen.flip();

	loop {
		match sdl::event::poll_event() {
			sdl::event::QuitEvent => break,
			sdl::event::NoEvent => {},
			sdl::event::KeyEvent(k, _, _, _) if k == sdl::event::EscapeKey => break,
			_ => {}
		}
	}

	sdl::quit();
}
