// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use strum::IntoEnumIterator;
use alc::{keyboard::layout_presets::{get_all_layout_size_presets, get_size_variant}, optimizer::{config::LayoutOptimizerTomlAdapter, optimize_from_toml, LayoutOptimizer}, text_processor::keycode::Keycode};
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
		process_config])
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
		.map(|x| x.to_string())
		.collect()
}

#[tauri::command]
fn process_config(config_file: String) {
	println!("received {} as the config file", config_file);
	let lo = match LayoutOptimizerTomlAdapter::try_from_toml_file(config_file.as_str()) {
		Ok(v) => v,
		Err(e) => {
			println!("{}", e);
			return;
		},
	};
	let size_variant = match get_size_variant((lo.layout_info.num_rows, lo.layout_info.num_cols)) {
		Ok(v) => v,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};
	// match optimize_from_toml(config_file) {
	// 	Ok(v) => v,
	// 	Err(e) => println!("{}", e),
	// }
}