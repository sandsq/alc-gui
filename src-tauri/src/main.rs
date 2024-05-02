// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use alc::keyboard::layout_presets::get_all_layout_size_presets;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_layout_presets])
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