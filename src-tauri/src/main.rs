// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf};

use strum::IntoEnumIterator;
use alc::{alc_error::AlcError, keyboard::{key::PhalanxKey, layer::Layer, layout::Layout, layout_presets::{get_all_layout_size_presets, get_size_variant}}, optimizer::{config::{DatasetOptions, GeneticOptions, LayoutInfoTomlAdapter, LayoutOptimizerTomlAdapter}, optimize_from_toml, LayoutOptimizer}, text_processor::keycode::{generate_default_keycode_set, Keycode, KeycodeOptions}};
use alc::keyboard::layout_presets::LayoutSizePresets::*;
use tauri::{api::path::config_dir, Manager};

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
		write_toml,
		create_blank_layers,
		get_config_dir,
		get_default_genetic_options,
		get_default_keycode_options,
		get_default_dataset_options,])
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
fn create_blank_layers(r: usize, c: usize, loc: String) -> Result<(String, String), AlcError> {
	let size_variant = get_size_variant((r, c))?;
	let (effort_layer, phalanx_layer) = match size_variant {
		TwoByFour => {
			(format!("{}", Layer::<2, 4, f64>::default()), format!("{}", Layer::<2, 4, PhalanxKey>::default()))
		}, 
		FourByTen => {
			(format!("{}", Layer::<4, 10, f64>::default()), format!("{}", Layer::<4, 10, PhalanxKey>::default()))
		},
		FourByTwelve => {
			(format!("{}", Layer::<4, 12, f64>::default()), format!("{}", Layer::<4, 12, PhalanxKey>::default()))
		}
		FiveByFifteen => {
			(format!("{}", Layer::<5, 15, f64>::default()), format!("{}", Layer::<5, 15, PhalanxKey>::default()))
		}
	};
	println!("received request to get blank effort and phalanx layers from {}:\n{} {}", loc, effort_layer, phalanx_layer);
	Ok((effort_layer, phalanx_layer))
}

#[tauri::command]
fn write_toml(filename: &str, layout_info: LayoutInfoTomlAdapter, num_threads: usize, genetic_options: String, keycode_options: String, dataset_options: String) -> Result<(), AlcError> {
	println!("writing {} with {:?}, {:?}", filename, layout_info, genetic_options);
	let toml = format!("\
	[layout_info]\n\
	num_rows = {}\n\
	num_cols = {}\n\
	layout = \"\"\"\n\
	{}\n\
	\"\"\"\n\
	effort_layer = \"\"\"\n\
	{}\n\
	\"\"\"\n\
	phalanx_layer = \"\"\"\n\
	{}\n\
	\"\"\"\n\
	[layout_optimizer_config]\n\
	valid_keycodes = []\n\
	num_threads = {}\n\
	[layout_optimizer_config.genetic_options]\n\
	{}\n\
	[layout_optimizer_config.keycode_options]\n\
	{}\n\
	[layout_optimizer_config.dataset_options]\n\
	{}\n\
	", layout_info.num_rows, layout_info.num_cols, layout_info.layout, layout_info.effort_layer, layout_info.phalanx_layer, num_threads,  genetic_options, keycode_options, dataset_options);
	fs::write(filename, toml).unwrap_or_else(|_| panic!("unable to write file {}", filename));
	Ok(())
}

#[tauri::command]
fn get_config_dir() -> Result<String, AlcError> {
	let d = config_dir().unwrap();
	let mut alc_dir = d.into_os_string();
	alc_dir.push("/alc/");
	match fs::create_dir_all(alc_dir.clone()) {
		Ok(v) => v,
		Err(_e) => return Err(AlcError::ExpectedDirectoryError(PathBuf::from(alc_dir)))
	}
	Ok(alc_dir.into_string().unwrap())
}

#[tauri::command]
fn get_default_genetic_options() -> GeneticOptions {
	GeneticOptions::default()
}

#[tauri::command]
fn get_default_keycode_options() -> (KeycodeOptions, Vec<Keycode>) {
	let mut valid_keycodes: Vec<Keycode> = generate_default_keycode_set(&KeycodeOptions::default()).into_iter().collect();
	valid_keycodes.sort();
	(KeycodeOptions::default(), valid_keycodes)
}

#[tauri::command]
fn get_default_dataset_options() -> DatasetOptions {
	DatasetOptions::default()
}