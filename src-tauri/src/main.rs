// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use strum::IntoEnumIterator;
use alc::{alc_error::AlcError, keyboard::layout_presets::{get_all_layout_size_presets, get_size_variant}, optimizer::{config::LayoutOptimizerTomlAdapter, optimize_from_toml, LayoutOptimizer}, text_processor::keycode::Keycode};
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
		app.listen_global("selected_toml_changed", |event| {
			let toml_file = event.payload();
			println!("toml file {}", toml_file.unwrap());
		});
		// {
		// let id = app.listen_global("selected_toml_changed", |event| {
		// 	println!("got event with payload {:?}", event.payload());
		// });
		Ok(())
	})
    .invoke_handler(tauri::generate_handler![
		greet, 
		get_layout_presets,
		get_all_keycodes,
		process_config,
		write_toml,])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}!", name)
}

#[tauri::command]
fn get_layout_presets() -> Vec<(usize, usize)> {
	// let mut out: HashMap<usize, (usize, usize)> = Default::default();
	let sizes = get_all_layout_size_presets();
	// for (i, size) in sizes.iter().enumerate() {
	// 	out.insert(i, *size);
	// }
	sizes
}

#[tauri::command]
fn get_all_keycodes() -> Vec<String> {
	Keycode::iter()
		.filter(|x| *x != Keycode::_PLACEHOLDER)
		.map(|x| x.to_string().replace('_', ""))
		.collect()
}

// define the payload struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Payload {
    message: String,
	pass: bool,
}
impl Payload {
	fn new(message: String, pass: bool) -> Self {
		Payload { message, pass }
	}
}

#[tauri::command]
fn process_config(app_handle: tauri::AppHandle, config_file: String) -> Result<LayoutOptimizerTomlAdapter, AlcError> {
// Result<LayoutOptimizerTomlAdapter, AlcError> {
	println!("received {} as the config file", config_file);
	let lo = LayoutOptimizerTomlAdapter::try_from_toml_file(config_file.as_str())?;
	// {
	// 	Ok(v) => v,
	// 	Err(e) => {
	// 		println!("{}", e);
	// 		app_handle.emit_all("config-error", Payload::new(e.to_string(), false)).unwrap();
	// 		return;
	// 	},
	// };
	let r = lo.layout_info.num_rows;
	let c = lo.layout_info.num_cols;
	let size_variant = get_size_variant((lo.layout_info.num_rows, lo.layout_info.num_cols))?;
	// {
	// 	Ok(v) => v,
	// 	Err(e) => {
	// 		println!("{}", e);
	// 		app_handle.emit_all("config-error", Payload::new(e.to_string(), false)).unwrap();
	// 		return;
	// 	}
	// };
	// match optimize_from_toml(config_file) {
	// 	Ok(v) => v,
	// 	Err(e) => println!("{}", e),
	// }
	Ok(lo)
}

#[tauri::command]
fn write_toml(filename: &str, p: Payload) -> Result<(), AlcError> {
	println!("writing {} with {:?}", filename, p);
	fs::write(filename, p.message).unwrap_or_else(|_| panic!("unable to write file {}", filename));
	Ok(())
}