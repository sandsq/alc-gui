// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, fs,  path::{Path, PathBuf}};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use strum::IntoEnumIterator;
use alc::{alc_error::AlcError, keyboard::{key::PhalanxKey, layer::Layer, layout_presets::{get_all_layout_size_presets, get_size_variant}}, optimizer::{config::{option_descriptions, DatasetOptions, GeneticOptions, LayoutInfoTomlAdapter, LayoutOptimizerConfig, LayoutOptimizerTomlAdapter, ScoreOptions}, optimize_from_toml, score_from_toml}, text_processor::keycode::{generate_default_keycode_set, Keycode, KeycodeOptions}};
use alc::keyboard::layout_presets::LayoutSizePresets::*;
use tauri::{api::path::config_dir, Manager};

fn main() {
	// here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	let close = CustomMenuItem::new("close".to_string(), "Close");
	let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
	let _menu = Menu::new()
	.add_submenu(submenu)
	.add_native_item(MenuItem::Copy);

	// std::panic::set_hook(Box::new(|info| {
    //     error!("Panicked: {:?}", info);
    // }));
  tauri::Builder::default()
	// .menu(menu)
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
		get_cache_dir,
		read_current_step_cache,
		get_default_genetic_options,
		get_default_keycode_options,
		get_default_dataset_options,
		recompute_valid_keycodes,
		get_default_score_options,
		does_file_exist,
		run_optimizer,
		get_option_descriptions,
		compute_score,])
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

// _app_handle: tauri::AppHandle, 
#[tauri::command]
fn process_config(config_file: String) -> Result<LayoutOptimizerTomlAdapter, AlcError> {
// Result<LayoutOptimizerTomlAdapter, AlcError> {
	println!("received {} as the config file to process", config_file);
	let lo: LayoutOptimizerTomlAdapter;
	if Path::new(config_file.as_str()).exists() {
		lo = LayoutOptimizerTomlAdapter::try_from_toml_file(config_file.as_str())?;
	} else {
		lo = LayoutOptimizerTomlAdapter::try_from_toml_string(config_file.as_str())?;
	}
	
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
	let _size_variant = get_size_variant((r, c))?;
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
		FiveBySix => {
			(format!("{}", Layer::<5, 6, f64>::default()), format!("{}", Layer::<5, 6, PhalanxKey>::default()))
		},
		FourByTen => {
			(format!("{}", Layer::<4, 10, f64>::default()), format!("{}", Layer::<4, 10, PhalanxKey>::default()))
		},
		FourByTwelve => {
			(format!("{}", Layer::<4, 12, f64>::default()), format!("{}", Layer::<4, 12, PhalanxKey>::default()))
		},
		FiveByTwelve => {
			(format!("{}", Layer::<5, 12, f64>::default()), format!("{}", Layer::<5, 12, PhalanxKey>::default()))
		},
		FiveByFifteen => {
			(format!("{}", Layer::<5, 15, f64>::default()), format!("{}", Layer::<5, 15, PhalanxKey>::default()))
		},
		SixByTwenty => {
			(format!("{}", Layer::<6, 20, f64>::default()), format!("{}", Layer::<6, 20, PhalanxKey>::default()))
		},
	};
	println!("received request to get blank effort and phalanx layers from {}:\n{} {}", loc, effort_layer, phalanx_layer);
	Ok((effort_layer, phalanx_layer))
}

#[tauri::command]
fn write_toml(filename: &str, layout_info: LayoutInfoTomlAdapter, num_threads: usize, genetic_options: GeneticOptions, keycode_options: KeycodeOptions, dataset_options: DatasetOptions, score_options: ScoreOptions) -> Result<(), AlcError> {
	println!("writing {}", filename);
	let layout_optimizer_config = LayoutOptimizerConfig {
		genetic_options,
		keycode_options,
		valid_keycodes: vec![],
		dataset_options,
		score_options,
		num_threads,
	};
	let adapter = LayoutOptimizerTomlAdapter {
		layout_info,
		layout_optimizer_config,
	};
	adapter.write_to_file(filename)?;
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
fn get_cache_dir() -> Result<String, AlcError> {
	let mut cache_dir = dirs::cache_dir().unwrap().into_os_string();
	cache_dir.push("/alc/");
	match fs::create_dir_all(cache_dir.clone()) {
		Ok(v) => v,
		Err(_e) => return Err(AlcError::ExpectedDirectoryError(PathBuf::from(cache_dir)))
	}
	Ok(cache_dir.into_string().unwrap())
}

#[tauri::command]
fn read_current_step_cache() -> Result<String, AlcError> {
	let cache_dir = get_cache_dir()?;
	let current_step_file = format!("{}current_step.txt", cache_dir);
	let contents = match fs::read_to_string(&current_step_file) {
		Ok(c) => c,
		Err(_) => {
			"".to_string()
		}
	};
	Ok(contents)
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
fn recompute_valid_keycodes(options: KeycodeOptions) -> Vec<Keycode> {
	let mut valid_keycodes: Vec<Keycode> = generate_default_keycode_set(&options).into_iter().collect();
	valid_keycodes.sort();
	valid_keycodes
}


#[tauri::command]
fn get_default_dataset_options() -> DatasetOptions {
	DatasetOptions::default()
}

#[tauri::command]
fn get_default_score_options() -> ScoreOptions {
	ScoreOptions::default()
}

#[tauri::command]
fn does_file_exist(filename: String) -> bool {
	Path::new(&filename).exists()
}

#[tauri::command]
async fn run_optimizer(filename: String) -> Result<String, AlcError> {
	let cache_dir = get_cache_dir()?;
	let current_step_file = format!("{}current_step.txt", cache_dir);
	fs::write(&current_step_file, "").unwrap();
	let final_path = optimize_from_toml(filename)?;
	fs::write(&current_step_file, "").unwrap();
	Ok(final_path)
}

#[tauri::command]
fn get_option_descriptions() -> HashMap<String, String> {
	option_descriptions()
}

#[tauri::command]
async fn compute_score(config_file: String) -> Result<f64, AlcError> {
	score_from_toml(config_file)
}