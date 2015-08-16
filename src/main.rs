extern crate sdl2;
extern crate gl;
extern crate rustc_serialize;

mod settings;

use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use sdl2::EventPump;

pub fn main() {
	
	let setts = settings::read_settings("settings.json");

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	
	let attrs = video_subsystem.gl_attr();
	
	attrs.set_double_buffer(true);
	attrs.set_context_profile(GLProfile::Core);
	attrs.set_context_version(setts.gl.major, setts.gl.minor);
	
	video_subsystem.gl_set_swap_interval(1);
	
	let window = video_subsystem.window(&*setts.window.title, setts.window.width, setts.window.height)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	
	let context = window.gl_create_context().unwrap();
		
	gl::load_with(|name| video_subsystem.gl_get_proc_address(name));

	let mut event_pump = sdl_context.event_pump().unwrap();

	while handle_events(&mut event_pump) {
		unsafe
		{
			gl::ClearColor(1.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		window.gl_swap_window();
	}
}

fn handle_events(event_pump : &mut EventPump) -> bool
{
	let mut running = true;
	for event in event_pump.poll_iter() {
		use sdl2::event::Event;

		match event {
			Event::Quit {..} => {
				running = false
			},
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				running = false
			},
			_ => {}
		}
	}
	running
}