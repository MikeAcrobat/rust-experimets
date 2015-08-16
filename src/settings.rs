use rustc_serialize::json;
use std::fs::File;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
pub struct ContextSettings {
	pub major : u8,
	pub minor : u8
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct WindowSettings {
	pub title	: String,
	pub width	: u32,
	pub height	: u32
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct AppSettings {
	pub window	: WindowSettings,
	pub gl		: ContextSettings
}

fn default_settings() -> AppSettings {
	AppSettings {
		window 		: WindowSettings {
			title 	: "Test".to_string(),
			width 	: 800,
			height 	: 600,
		},
		gl		 	: ContextSettings {
			major 	: 3,
			minor 	: 0,
		},
	}
}

pub fn read_settings(file_path : &str) -> AppSettings {
	let mut content = String::new();

	if let Ok(mut file) = File::open(file_path) {
		let _ = file.read_to_string(&mut content);
	}

	match json::decode(&content) {
		Ok(settings) => settings,
		Err(_) => default_settings()
	}
}