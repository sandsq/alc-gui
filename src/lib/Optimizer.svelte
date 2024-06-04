<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount, onDestroy } from 'svelte';
	import { open, save, ask } from '@tauri-apps/api/dialog';
	import { setContext } from 'svelte';
	import Layout from './Layout.svelte';
	import EffortLayer from './EffortLayer.svelte';
	import PhalanxLayer from './PhalanxLayer.svelte';
	import Help from "./Help.svelte"
	import { appWindow } from '@tauri-apps/api/window';
	import { layer_switch_regex } from './utils';
	import SvelteMarkdown from 'svelte-markdown';
	import { Tooltip, tooltip } from "@svelte-plugins/tooltips";
	import { fade, slide } from 'svelte/transition'
	import { copy } from "svelte-copy";
	import { hide } from '@tauri-apps/api/app';
	

	// let visible = false

	// const keycode_options_info = "Options to specify the set of keycodes to which text should be translated; included in this specification are which keycodes should be treated as shifted versions of their base keycodes (e.g., should \"plus\" get its own keycode or should it be treated as \"shift + equals\"). The shift key itself and all non-shifted keycodes need to appear in the layout or some ngrams won't be typeable."
	const keycode_list_info = "List of keycodes constructed from the given options. Unless a given keycode is already present in the layout, it will be randomly placed into an empty slot in the layout to form the first generation. Dataset text is translated into these keycodes. This means that if the user wishes to use, for example, \"&\" (AMPR) without needing to type \"shift + 7\", AMPR should appear in this list. Or another way, if AMPR is NOT in this list, then dataset text containing \"&\" will be translated to \"SFT + 7\". Thus, key placement optimization would only be performed on SFT and 7, not AMPR."
	const convenience_info = "These options cover base, non-shifted keycodes. If toggled off, the user must manually place each alpha and misc symbol themselves. If toggled on, the optimizer will automatically place said symbols, hence, convenience."
	const specific_info = "These options cover how text should be translated into keycodes, e.g., should \"plus\" be treated as its own keycode or should it be treated as \"shift and equals\"."

	const hide_flags_info = "Hide flags for a cleaner view of the layout."

	/**@type {Object.<string, string>}*/
	let option_descriptions;

	/**@type {keyof TabComponentMap}*/
	let active_tab = 'tab1';

	/**@type boolean*/
	let hide_flags = false;

	/**
	 * @typedef {Object} TabComponentMap
	 * @property {typeof Layout} tab1
	 * @property {typeof EffortLayer} tab2
	 * @property {typeof PhalanxLayer} tab3
	 * @property {typeof Help} tab4
	 * @property {string} tab5
	 */
	/** @type {TabComponentMap} */
	const tab_components = {
		tab1: Layout,
		tab2: EffortLayer,
		tab3: PhalanxLayer,
		tab4: Help,
		tab5: "",
	};

	/** @type {string | string[] | null}*/
	let selected_toml_file;
	/**@type {string}*/
	let layout_string;
	/**@type {string}*/
	let effort_layer_string;
	/**@type {string}*/
	let phalanx_layer_string;

	/**@type {number}*/
	let num_threads = 1;
	/**@type {any}*/
	let genetic_options;
	/**@type {string[]}*/
	let valid_keycodes = [];
	/**@type {any}*/
	let dataset_options;

	/**
	 * @typedef {Object} KeycodeOptions
	 * @type {Record<string, boolean | string[]>}}
	 * @property {boolean} include_alphas
	 * @proprety {boolean} include_numbers
	 * @property {boolean} include_number_symbols
	 * @proprety {boolean} include_brackets
	 * @property {boolean} include_misc_symbols
	 * @property {boolean} include_misc_symbols_shifted
	 * @property {string[]} explicit_inclusions
	 */
	// /**@type {KeycodeOptions}*/
	/**@type {any}*/
	let keycode_options;
	/**@type {any}*/
	let score_options;

	/**@param {boolean} show_dialog
	 * @param {string} content_to_open
	*/
	async function open_toml(show_dialog, content_to_open) {
		/**@type {string | string[] | null}*/
		let opened_file;
		if (show_dialog) {
			opened_file = await open({
				multiple: false,
				filters: [
					{
						name: 'toml',
						extensions: ['toml']
					}
				]
			});
		} else {
			opened_file = content_to_open;
		}
		if (opened_file !== null) {
			await invoke('process_config', { configFile: opened_file })
				.then((res) => {
					is_size_from_config = true;
					selected_toml_file = opened_file;
					selected_size = [res.layout_info.num_rows, res.layout_info.num_cols];
					selected_num_layers = (res.layout_info.layout.match(/Layer/g) || []).length;
					layout_string = res.layout_info.layout;
					effort_layer_string = res.layout_info.effort_layer;
					phalanx_layer_string = res.layout_info.phalanx_layer;
					console.log(`loaded phalanx layer string ${phalanx_layer_string}`);
					num_threads = res.layout_optimizer_config.num_threads;
					genetic_options = res.layout_optimizer_config.genetic_options;
					keycode_options = res.layout_optimizer_config.keycode_options;
					recompute_valid_keycodes();
					dataset_options = res.layout_optimizer_config.dataset_options;
					score_options = res.layout_optimizer_config.score_options;
					// is_size_from_config = false
					saved = false
					console.log(`successfully loaded ${opened_file}`);
				})
				.catch((e) => {
					alert(e);
					console.error(e);
				});
			compute_score()
			// console.log(selected_file)
			// emit("selected_toml_changed", toml_file)
		}

	}

	/** @type {[number, number][]}*/
	let layout_sizes = [];
	/** @type {[number, number]}*/
	let selected_size; //= [2, 4];
	/** @type {boolean}*/
	let is_size_from_config = false;
	/** @type {number}*/
	let selected_num_layers = 3;
	let max_layers = 9;

	// /**
	// * @typedef {Object} Key
	// * @property {string} keycode
	// * @property {boolean} locked
	// * @property {boolean} symmetric
	// */

	/**
	 * @constructor
	 * @param {string} keycode
	 * @param {boolean} locked
	 * @param {boolean} symmetric
	 */
	function Key(keycode, locked, symmetric) {
		this.keycode = keycode;
		this.locked = locked;
		this.symmetric = symmetric;
	}
	Key.prototype.toString = function () {
		return `${this.keycode}_${+!this.locked}${+this.symmetric}`;
	};

	/** @type {Key[][][]}*/
	let layout;
	/** @type {number[][]}*/
	let effort_layer;
	/** @type {[string, string][][]}*/
	let phalanx_layer;

	/** @type {string[]}*/
	let all_keycodes = [];

	async function get_sizes() {
		layout_sizes = await invoke('get_layout_presets');
		// selected_size = layout_sizes[0]
	}
	async function get_keycodes() {
		all_keycodes = await invoke('get_all_keycodes');
		all_keycodes = all_keycodes.filter((x) => x != 'LS' && x != 'LST');
	}
	$: {
		selected_num_layers;
		all_keycodes = all_keycodes.filter((str) => {
			const regex = /^LS\d+$/;
			return !regex.test(str);
		});
		for (let n = 0; n < selected_num_layers; n++) {
			all_keycodes.push(`LS${n}`);
		}
	}

	// let active_tab_color = "#FBF1C7"
	// let active_tab_border = "#d65d0e"
	// let inactive_tab_border = "#847b6b"

	/**@param {number} c
	 * @param {boolean} is_key
	 * 7 and 4 are numbers from the alc backend
	 */
	function col_indexes_to_string(c, is_key) {
		let output = '   ';
		for (let k = 0; k < c; k++) {
			if (is_key) {
				output += k.toString().padStart(7);
			} else {
				output += k.toString().padStart(4);
			}
			output += ' ';
		}
		return output;
	}

	/**
	 * @param {number[][] | [string, string][][]} layer
	 */
	function layer_to_string(layer) {
		let output = '';
		let col_inds = col_indexes_to_string(layer[0].length, false);
		output += col_inds;
		output += '\n';
		for (let i = 0; i < layer.length; i++) {
			output += `${i}|`;
			for (let j = 0; j < layer[0].length; j++) {
				let v = layer[i][j];
				if (Array.isArray(v)) {
					output += `${v[0][0]}:${v[1][0]}`.padStart(4);
				} else {
					output += v.toString().padStart(4);
				}
			}
			output += ' \n';
		}
		return output;
	}

	/**
	 * @param {Key[][]} layer
	 * @param {number} layer_num
	 */
	function keycode_layer_to_string(layer, layer_num) {
		let output = '';
		let col_inds = col_indexes_to_string(layer[0].length, true);
		output += col_inds;
		output += '\n';
		for (let i = 0; i < layer.length; i++) {
			output += `${i}|`;
			for (let j = 0; j < layer[0].length; j++) {
				let v = layer[i][j];
				let keycode_string = v.keycode;
				if (layer_switch_regex.test(v.keycode)) {
					const regex = /^LS(\d+)$/;
					const match = v.keycode.match(regex);
					let corresponding_layer = -1;
					if (match) {
						corresponding_layer = parseInt(match[1], 10);
						// we have a layer switch, but we are in the target layer for that layer switch. So, we should replace it with _
						if (layer_num == corresponding_layer) {
							keycode_string = '_';
						}
					} else {
						console.error(
							`problem converting the X in LSX to a number ${v.keycode}, probably a developer parsing error`
						);
					}
				}
				output += `${v.keycode}_${+!v.locked}${+v.symmetric}`.padStart(8);
			}
			output += ' \n';
		}
		return output;
	}

	/**@param {Key[][][]} layout*/
	function layout_to_string(layout) {
		// for (let n = 0; n < layout.length; n++) {
		// 	let layer = layout[n]
		// 	for (let i = 0; i < layer.length; i++) {
		// 		let row = layer[i]
		// 		for (let j = 0; j < row.length; j++) {
		// 			let col = row[j]
		// 			if (col.keycode == "LS") {

		// 			}
		// 		}
		// 	}
		// }
		let output = '';
		for (let n = 0; n < layout.length; n++) {
			output += `___Layer ${n}___\n`;
			let layer = keycode_layer_to_string(layout[n], n);
			output += layer;
		}
		return output;
	}

	let saved = true;
	/**
	 * @typedef {Object} LayoutInfo
	 * @property {number} num_rows
	 * @property {number} num_cols
	 * @property {string} layout
	 * @property {string} effort_layer
	 * @property {string} phalanx_layer
	 */

	/**@param {boolean} show_saving_dialog
	 * @param {boolean} should_autosave
	 */

	async function write_toml(show_saving_dialog, should_autosave) {
		/**@type {string | null}*/
		let save_path;
		if (show_saving_dialog) {
			save_path = await save({
				filters: [
					{
						name: 'toml',
						extensions: ['toml']
					}
				],
				defaultPath: `${config_dir}hello.toml`
			});
		} else {
			save_path = `${config_dir}autosave.toml`;
		}
		if (should_autosave) {
			save_path = `${config_dir}autosave.toml`;
		}

		/**@type LayoutInfo*/
		let li = {
			num_rows: selected_size[0],
			num_cols: selected_size[1],
			layout: layout_to_string(layout),
			effort_layer: layer_to_string(effort_layer),
			phalanx_layer: layer_to_string(phalanx_layer)
		};
		try {
			if (save_path) {
				await invoke('write_toml', {
					filename: save_path,
					numThreads: num_threads,
					layoutInfo: li,
					geneticOptions: genetic_options,
					keycodeOptions: keycode_options,
					datasetOptions: dataset_options,
					scoreOptions: score_options
				});
				saved = true
			}
		} catch (e) {
			alert(e);
		}
	}

	/**
	 * @param {[number, number]} size
	 * @param {string} test*/
	async function create_blank_layers(size, test) {
		if (size && !is_size_from_config) {
			console.log(`creating blank layers ${test}`);
			await invoke('create_blank_layers', { r: size[0], c: size[1], loc: test })
				.then((res) => {
					effort_layer_string = res[0];
					phalanx_layer_string = res[1];
				})
				.catch((e) => {
					alert(e);
					console.error(e);
				});
		}
	}
	$: {
		create_blank_layers(selected_size, 'from $: selected_size, ...');
	}

	async function get_default_genetic_options() {
		await invoke('get_default_genetic_options')
			.then((res) => {
				genetic_options = res;
			})
			.catch((e) => {
				alert(e);
				console.error(e);
			});
	}
	async function get_default_keycode_options() {
		await invoke('get_default_keycode_options')
			.then((res) => {
				keycode_options = res[0];
				valid_keycodes = res[1];
			})
			.catch((e) => {
				alert(e);
				console.error(e);
			});
	}
	async function recompute_valid_keycodes() {
		// console.log(`explicit inclusions ${keycode_options.explicit_inclusions}`)
		await invoke('recompute_valid_keycodes', { options: keycode_options })
			.then((res) => {
				valid_keycodes = res;
				keycode_display = valid_keycodes;
			})
			.catch((e) => {
				alert(e);
				console.error(e);
			});
	}
	async function get_default_dataset_options() {
		await invoke('get_default_dataset_options')
			.then((res) => {
				dataset_options = res;
			})
			.catch((e) => {
				alert(e);
				console.error(e);
			});
	}
	async function get_default_score_options() {
		await invoke('get_default_score_options')
			.then((res) => {
				score_options = res;
			})
			.catch((e) => {
				alert(e);
				console.error(e);
			});
	}

	let options_display = 'options_inline';

	function readjust_tab_contents() {
		/**@type {HTMLElement | null}*/
		let layout_element = document.querySelector('.layout');
		/**@type {HTMLElement | null}*/
		let options_inline_element = document.querySelector('options_inline');
		/**@type {HTMLElement | null}*/
		let options_block_element = document.querySelector('options_block');
		if (
			layout_element !== null &&
			options_inline_element !== null &&
			options_block_element !== null
		) {
			if (layout_element.offsetWidth >= options_inline_element.offsetWidth) {
				options_display = 'options_block';
			} else {
				options_display = 'options_inline';
			}
		}
	}

	let explicit_inclusion_string = '';
	/**@type {string[]} */
	let keycode_display = [];

	$: {
		if (keycode_options) {
			keycode_options.explicit_inclusions, recompute_valid_keycodes();
		}
	}

	function add_explicit_inclusion() {
		if (keycode_options) {
			console.log('adding inclusion option');
			keycode_options.explicit_inclusions.push('_NO');
			keycode_options.explicit_inclusions = keycode_options.explicit_inclusions;
			recompute_valid_keycodes();
			compute_score();
		}
	}
	/**@param {number} ind */
	function remove_explicit_inclusion(ind) {
		if (keycode_options) {
			console.log('removing inclusion option');
			keycode_options.explicit_inclusions.splice(ind, 1);
			keycode_options.explicit_inclusions = keycode_options.explicit_inclusions;
			recompute_valid_keycodes();
			compute_score()
		}
	}

	async function add_dataset_path() {
		if (dataset_options) {
			/**@type {string | string[] | null}*/
			let opened_dir;
			opened_dir = await open({
				multiple: true,
				directory: true
			});
			if (opened_dir) {
				console.log(`adding ${opened_dir}`)
				if (Array.isArray(opened_dir)) {
					for (let dir of opened_dir) {
						dataset_options.dataset_paths.push(dir)
						dataset_options.dataset_paths = dataset_options.dataset_paths

						dataset_options.dataset_weights.push(1.0)
						dataset_options.dataset_weights = dataset_options.dataset_weights	
					}
				} else {
					dataset_options.dataset_paths.push(opened_dir)
					dataset_options.dataset_paths = dataset_options.dataset_paths

					dataset_options.dataset_weights.push(1.0)
					dataset_options.dataset_weights = dataset_options.dataset_weights
				}
				saved = false
				// compute_score()
			}
		}
	}
	
	/**@param {number} i*/
	function remove_dataset_path(i) {
		if (dataset_options) {
			dataset_options.dataset_paths.splice(i, 1)
			dataset_options.dataset_paths = dataset_options.dataset_paths

			dataset_options.dataset_weights.splice(i, 1)
			dataset_options.dataset_weights = dataset_options.dataset_weights

			saved = false
			// compute_score()
		}
	}

	/**@type {string[]}*/
	let presets = ["5x6_left", "5x6_right", "4x10", "ferris_sweep", "4x12", "5x12", "5x15", "6x20"]
	/**@param {string} preset_name*/
	async function fetch_preset_by_name(preset_name) {
		fetch(`${preset_name}.toml`).then((res) => {
			if (res.ok) {
				res.text().then((t) => {
					open_toml(false, t)
				})
			} else {
				alert("preset doesn't exist")
			}
		})
	}

	let optimizing_wait_messages = ["Optimizing.  ", "Optimizing.. ", "Optimizing..."]
	/**@type {number}*/
	let roller;
	let index = 0;
	let help_doc = "";
	
	/**@type {any}*/
	let current_step;

	let config_dir = '';
	/**@type {number}*/
	let save_interval_timer;

	let autosave_location = "";
	onMount(() => {
		roller = setInterval(() => {
			if (index === optimizing_wait_messages.length - 1) {
				index = 0;
			} else {
				index++;
			}
			invoke("read_current_step_cache").then((res) => {
				current_step = res
			})
		}, 1000);
		invoke('get_config_dir').then((res) => {
			config_dir = res;
			let autosaved_file = `${config_dir}autosave.toml`;
			autosave_location = autosaved_file
			// selected_toml_file = 
			invoke('does_file_exist', { filename: autosaved_file }).then((res) => {
				if (res) {
					open_toml(false, autosaved_file).then((res) => {
						console.log(JSON.stringify(layout));
						console.log(`size from config ${is_size_from_config}`);
					});
				}
			});
		});
		get_sizes().then((res) => {
			selected_size = layout_sizes[0];
		});
		get_keycodes();
		get_default_genetic_options();
		get_default_keycode_options().then((res) => {
			explicit_inclusion_string = keycode_options.explicit_inclusions.join(',');
			keycode_display = valid_keycodes;
		});
		get_default_dataset_options();
		get_default_score_options();
		invoke("get_option_descriptions").then((res) => {
			option_descriptions = res
		})

		save_interval_timer = setInterval(() => {
			if (!saved) {
				write_toml(false, true)
			}
		}, 10000);

		fetch("help.md").then((res) => {
			// console.log(`fetch help ${res}`)
			res.text().then((t) => {
				help_doc = t
			})
		})
		
		// appWindow.once("ready", async () => {
		// 	await create_blank_layers("app window ready")
		// selected_size = layout_sizes[0]
		// })
		
	});
	onDestroy(() => {
		clearInterval(save_interval_timer)
		clearInterval(roller)
	});

	
	let container;
	/**@param {any} event*/
	function handleChange(event) {
		const target = event.target;
		let value;
		if (target) {
			switch (target.type) {
				case 'checkbox':
					value = target.checked;
					break;
				case 'range':
					value = target.value;
					break;
				default:
					value = target.value;
					console.log(target.type);
			}
			console.log(`value is ${value}`)
			if (value) {
				saved = false;
				compute_score()
			}
			
		}
	}


	/**@type {Promise<number | void>}*/
	let current_score_promise;
	/**@type {number | string}*/
	let current_score;
	async function compute_score() {
		await write_toml(false, true)
		// current_score_promise = 
		invoke("compute_score", {configFile: `${config_dir}autosave.toml`}).then((res) => {
			console.log(res)
			current_score = res
		}).catch((e) => {
			current_score = e
		})
	}
	
	

	/**@type {Promise<void>}*/
	let optimizer_run;
	async function run_optimizer() {
		await write_toml(false, true)
		optimizer_run = invoke("run_optimizer", {filename: `${config_dir}autosave.toml`}).then(async (res) => {
			let show_final_result = await confirm(`Load best layout? A copy can be found at ${res}`)
			if (show_final_result) {
				// selected_toml_file = res
				open_toml(false, res)
				saved = false
				await compute_score()
			}
		}).catch((e) => {
			alert(`something went wrong: ${e}`)
			console.error(e)
		})
	}

	let layer_spec = "OSL"
	let copyable_layout = "";
	$: {
		if (layout) {
			copyable_layout = ""
			for (let n = 0; n < layout.length; n++) {
				for (let i = 0; i < layout[n].length; i++) {
					for (let j = 0; j < layout[n][i].length; j++) {
						let key = layout[n][i][j]
						let keycode = key.keycode
						if (keycode == "NO" && key.locked) {
							copyable_layout += "XXXXXXX".padStart(8)
						} else if (keycode == "NO") {
							copyable_layout += "_______".padStart(8)
						} else if (keycode == "ZERO") {
							copyable_layout += "KC_0".padStart(8)
						} else if (keycode.includes("LS")) {
							const regex = /^LS(\d+)$/;
							const match = keycode.match(regex);
							let corresponding_layer = -1;
							if (match) {
								corresponding_layer = parseInt(match[1], 10);
								if (n == corresponding_layer) {
									copyable_layout += "_______".padStart(8)
								} else {
									copyable_layout += `${layer_spec}(${corresponding_layer})`.padStart(8)
								}
							} else {
								copyable_layout += `KC_${keycode}`.padStart(8)	
							}
						} else {
							copyable_layout += `KC_${keycode}`.padStart(8)
						}
						if (i == layout[n].length - 1 && j == layout[n][i].length - 1) {

						} else {
							copyable_layout += ", "
						}
						
					}
					copyable_layout += "\n"
				}
				copyable_layout += "\n"
			}
		}
		
	}
	
</script>

<p>autosaved status: {saved} at {autosave_location}</p>

<div class="header_button">
	<button on:click={() => open_toml(true, "")}>Load config</button>
	<br />
	or choose preset
	{#each presets as preset}
		<button on:click={() => fetch_preset_by_name(preset)}>{preset}</button>&nbsp;
	{/each}
	<br />
	or choose size and layers:
	<select class="header_select"
		on:change={() => {
			is_size_from_config = false;
			saved = false;
		}}
		bind:value={selected_size}
		on:change={readjust_tab_contents}
	>
		{#each layout_sizes as size}
			<option value={size}>{size[0]} x {size[1]}</option>
		{/each}
	</select>
	<select class="header_select"
		on:change={() => {saved = false;}}
		bind:value={selected_num_layers}>
		{#each { length: max_layers } as _, i}
			<option value={i + 1}>{i + 1}</option>
		{/each}
	</select>
	<!-- <button on:click={() => open_toml(false, ferris_sweep_preset)}>Load test</button> -->
</div>

<br />

<span class="header_button"><button on:click={() => {run_optimizer()}}>Optimize!</button></span>
{#await optimizer_run}
  <!-- <p transition:fade
     on:introstart="{() => visible = false}"
     on:outroend="{() => visible = true}">
  ...waiting </p> -->
  <!-- transition:slide -->
  <span>&nbsp; {optimizing_wait_messages[index]} {current_step}</span>
{:then value}
	<span>&nbsp; Not currently optimizing</span>
{/await}

<br />

{#if selected_size}
	<div class="tabs">
		<button
			on:click={() => (active_tab = 'tab1')}
			class="{active_tab == 'tab1' ? 'active_tab' : 'inactive_tab'} tab">Layout</button
		>
		<button
			on:click={() => (active_tab = 'tab2')}
			class="{active_tab == 'tab2' ? 'active_tab' : 'inactive_tab'} tab">Effort layer</button
		>
		<button
			on:click={() => (active_tab = 'tab3')}
			class="{active_tab == 'tab3' ? 'active_tab' : 'inactive_tab'} tab">Hand assignment</button
		>
		<button
			on:click={() => (active_tab = 'tab5')}
			class="{active_tab == 'tab5' ? 'active_tab' : 'inactive_tab'} tab">Copyable version</button
		>
		<button
			on:click={() => (active_tab = 'tab4')}
			class="{active_tab == 'tab4' ? 'active_tab' : 'inactive_tab'} tab">Help</button
		>
	</div>
	<div bind:this={container} on:change={handleChange}>
		<div class="tab_contents">
			<div class="layout">
				<div class={active_tab == 'tab1' ? 'tabshow' : 'tabhide'}>
					<svelte:component
						this={tab_components['tab1']}
						bind:layout
						bind:keycodes={all_keycodes}
						num_layers={selected_num_layers}
						layout_size={selected_size}
						{layout_string}
						{is_size_from_config}
						bind:hide_flags
						bind:saved
					/>
				</div>
				<div class={active_tab == 'tab2' ? 'tabshow' : 'tabhide'}>
					<svelte:component
						this={tab_components['tab2']}
						bind:effort_layer_string
						bind:effort_layer
						bind:layout_size={selected_size}
						{is_size_from_config}
					/>
				</div>
				<div class={active_tab == 'tab3' ? 'tabshow' : 'tabhide'}>
					<svelte:component
						this={tab_components['tab3']}
						bind:phalanx_layer_string
						bind:phalanx_layer
						bind:layout_size={selected_size}
						{is_size_from_config}
					/>
				</div>
				<div class={active_tab == "tab5" ? "tabshow" : "tabhide"}>
					<button style="margin-top: 10px;" use:copy={copyable_layout}>Copy!</button>
					<select bind:value={layer_spec}>
						<option value="OSL">OSL</option>
						<option value="MO">MO</option>
					</select>
					<code><pre>{copyable_layout}</pre></code>
				</div>
				<div class={active_tab == "tab4" ? "tabshow" : "tabhide"}>
					{#if help_doc}
						<div class="help_doc">
						<svelte:component
							this={tab_components["tab4"]}
						/>
						<!-- source={help_doc} -->
						</div>
					{/if}
				</div>
			</div>

			{#if option_descriptions && active_tab != "tab4"}
				<div class="options {options_display} header_button">
					<div class="header_button" style="margin-top: 1rem;">
						<button on:click={() => hide_flags = !hide_flags}>{hide_flags ? "Show" : "Hide"} flags<span class="tooltip_div" use:tooltip={{ content: hide_flags_info, position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u>?</u></span></button>
					</div>
					<button on:click={() => write_toml(true, false)}>Save layout and options as</button>
					<br />
					<div style="width: 600px;"><button on:click={compute_score}>Compute score:</button> {typeof current_score == "number" ? current_score.toFixed(4) : current_score}</div>
					<h1>Options</h1>
					<h2>Keycode options</h2>
					<!-- <span class="tooltip_div" use:tooltip={{ content: keycode_options_info, position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></h2> -->
					<table>
						{#if keycode_options}
							<tr><th>Convenience options<span class="tooltip_div" use:tooltip={{ content: convenience_info, position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></th></tr>
							{#each ["include_alphas", "include_misc_symbols"] as key}
								<tr>
									<td>{key}<span class="tooltip_div" use:tooltip={{ content: option_descriptions[key], position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></td>
										<td>
											<label class="switch">
												<input
													type="checkbox"
													bind:checked={keycode_options[key]}
													on:change={recompute_valid_keycodes}
												/>
												<span class="slider round"></span>
											</label>
										</td>
									</tr>
							{/each}
							<tr><td colspan="2">
								As there currently is no way to constrain keycodes to appear<br />in certain orders with respect to each other, the user must<br />place numbers themselves.
							</td></tr>
							<tr><th>Specific options<span class="tooltip_div" use:tooltip={{ content: specific_info, position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></th></tr>
							{#each Object.entries(keycode_options) as [key, value]}
								{#if key == "include_alphas" || key == "include_misc_symbols"}
									<!-- -->
								{:else if key != "include_numbers"}
								<tr>
									<td>{key}<span class="tooltip_div" use:tooltip={{ content: option_descriptions[key], position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></td>
									<td>
										{#if key != 'explicit_inclusions'}
											<label class="switch">
												<input
													type="checkbox"
													bind:checked={keycode_options[key]}
													on:change={recompute_valid_keycodes}
												/>
												<span class="slider round"></span>
											</label>
											<!-- {key}: <input type="checkbox" bind:checked={keycode_options[key]} on:change={recompute_valid_keycodes} /> <br> -->
										{:else}
											<!-- {#key keycode_options.explicit_inclusions} -->
											{#each keycode_options.explicit_inclusions as code, ei_ind}
												<select
													bind:value={keycode_options.explicit_inclusions[ei_ind]}
													on:change={recompute_valid_keycodes}
												>
													{#each all_keycodes as keycode}
														<option value="_{keycode}">{keycode == 'NO' ? '' : keycode}</option>
													{/each}
												</select><button on:click={() => remove_explicit_inclusion(ei_ind)}>x</button>
											{/each}

											<button class="plus_button" on:click={add_explicit_inclusion}>+</button>
											<!-- {/key} -->
										{/if}
									</td>
								</tr>
								{/if}
							{/each}
							<tr>
								<td colspan="2">
								<div style="width: 750px; word-wrap: normal;">
									Keycode list<span class="tooltip_div" use:tooltip={{ content: keycode_list_info, position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u>?</u>&nbsp;</span><span style="margin-left: 20px;">{keycode_display.join(', ')}</span>
								</div>
								<td>
							</tr>
						{/if}
					</table>
					<table>
						<!-- <tr><th style="font-size: 32px;">Options</th></tr> -->
						
					</table>
					<h2>Dataset options</h2>
					<table>
						{#if dataset_options}
							<!-- <tr><th>Dataset options</th></tr> -->
							{#each Object.entries(dataset_options) as [key, value]}
								<tr>
									<td>{key}<span class="tooltip_div" use:tooltip={{ content: option_descriptions[key], position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></td>
									<td>
										{#if key == 'dataset_paths'}
											<div style="width: 400px; word-wrap: break-word;">
												{#each dataset_options['dataset_paths'] as dir, d_ind}
													{dir}<button class="plus_button" on:click={() => remove_dataset_path(d_ind)}>x</button>
												{/each}
												<button class="plus_button" on:click={add_dataset_path}>+</button>
											</div>
										{:else if key == "dataset_weights"}
											{#each dataset_options['dataset_weights'] as weight, d_ind}
												<input type="number" bind:value={weight} />
											{/each}
										{:else if key == "max_ngram_size"}
											<input
												type="range"
												min="1"
												max="5"
												step="1"
												bind:value={dataset_options[key]}
											/>
											<input type="number" bind:value={dataset_options[key]} />
										{:else if key == "top_n_ngrams_to_take"}
											<input
												type="range"
												min="0"
												max="100"
												step="10"
												bind:value={dataset_options[key]}
											/>
											<input type="number" bind:value={dataset_options[key]} />
										{:else}
											{value}
										{/if}
									</td>
								</tr>
							{/each}
						{/if}
					</table>
					<h2>Genetic options</h2>
					<table>
						{#if genetic_options}
							{@const mutation_total = genetic_options.swap_weight + genetic_options.replace_weight}
							<!-- <tr><th>Genetic options</th><th></th></tr> -->
							{#each Object.entries(genetic_options) as [key, value]}
								<tr>
									<td>{key}<span class="tooltip_div" use:tooltip={{ content: option_descriptions[key], position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>&nbsp;<u >?</u>&nbsp;</span></td>
									<td>
										{#if key == 'population_size'}
											<input
												type="range"
												min="0"
												max="10000"
												step="100"
												bind:value={genetic_options[key]}
											/>
											<input type="number" bind:value={genetic_options[key]} />
										{:else if key == 'generation_count'}
											<input
												type="range"
												min="0"
												max="500"
												step="10"
												bind:value={genetic_options[key]}
											/>
											<input type="number" bind:value={genetic_options[key]} />
										{:else if key == 'fitness_cutoff'}
											<input
												type="range"
												min="0"
												max="1"
												step="0.05"
												bind:value={genetic_options[key]}
											/>
											<input type="number" bind:value={genetic_options[key]} />
										{:else if key == 'swap_weight' || key == 'replace_weight'}
											<input
												type="range"
												min="0"
												max="10"
												step="1"
												bind:value={genetic_options[key]}
											/>
											<input type="number" bind:value={genetic_options[key]} />
										{:else}
											{value}
										{/if}
									</td>
								</tr>
							{/each}
							<tr><td colspan="2">
								{((genetic_options.swap_weight / mutation_total) * 100).toFixed(0)}% swaps, {((genetic_options.replace_weight / mutation_total) *
								100).toFixed(0)}% replacements
							</td></tr>
						{/if}
						<tr>
							<td>num_threads <u class="tooltip_div" use:tooltip={{ content: option_descriptions["num_threads"], position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 400 }}>?</u></td>
							<td>
								<input type="range" min="1" max="24" bind:value={num_threads} />
								<input type="number" min="1" max="24" bind:value={num_threads} />
							</td>
						</tr>
					</table>
					
					
					<h2>Scoring options</h2>
					<table>
						{#if score_options}
							<!-- <tr><th>Scoring options</th></tr> -->
							{#each Object.entries(score_options) as [key, value]}
								<tr>
									<td>{key == "finger_roll_same_row_reduction_factor" ? "same_row_reduction_factor" : key}<span class="tooltip_div" use:tooltip={{ content: option_descriptions[key], position: 'top', animation: 'slide', theme: "tooltip", maxWidth: 450 }}>&nbsp;<u >?</u>&nbsp;</span></td>
									<td>
										{#if key == "hand_alternation_weight" || key == "finger_roll_weight"}
											<input
												type="range"
												min="0"
												max="10"
												step="1"
												bind:value={score_options[key]}
											/>
											<input type="number" bind:value={score_options[key]} />
										{:else if key == "hand_alternation_reduction_factor" || key == "finger_roll_reduction_factor" || key == "finger_roll_same_row_reduction_factor"}
											<input
												type="range"
												min="0"
												max="1"
												step="0.05"
												bind:value={score_options[key]}
											/>
											<input type="number" min="0" max="1" step="0.05" bind:value={score_options[key]} />
										{:else}
											<input
												type="range"
												min="1"
												max="10"
												step="0.05"
												bind:value={score_options[key]}
											/>
											<input type="number" min="1" max="10" step="0.05" bind:value={score_options[key]} />
										{/if}
									</td>
								</tr>
								<tr>
									<td colspan="2">
									{#if key == "finger_roll_weight"}
										<div style="margin-top: 10px; margin-bottom: 10px;">The importance of hand alternation is <br /> {(score_options.hand_alternation_weight / score_options.finger_roll_weight).toFixed(2)}x that of finger rolling.</div>
									{/if}
									</td>
								</tr>
							{/each}
						{/if}
					</table>
				</div>
			{/if}
		</div>
	</div>
{/if}

<div class="debug">
	<h1>Debug section</h1>
	<p>size: {selected_size}</p>
	<p>file: {selected_toml_file}</p>
	<p>config dir: {config_dir}</p>
	<p>tab: {active_tab}</p>
	{#if layout}
		{#each layout as layer}
			<table>
				{#each layer as row}
					<tr>
						{#each row as col}
							<td>{col.keycode}_{+col.locked}{+col.symmetric}</td>
						{/each}
					</tr>
				{/each}
			</table>
		{/each}
	{/if}

	{#if effort_layer}
		<table>
			{#each effort_layer as row}
				<tr>
					{#each row as col}
						<td>{col}</td>
					{/each}
				</tr>
			{/each}
		</table>
	{/if}

	{#if phalanx_layer}
		<table>
			{#each phalanx_layer as row}
				<tr>
					{#each row as col}
						<td>{col.join(':')}</td>
					{/each}
				</tr>
			{/each}
		</table>
	{/if}
</div>


<style lang="scss">
	@use '../styles/colors.scss' as *;
	:global(*) {
		color: $text;
	}
	.header_button button {
		color: $red_dark;
		background-color: $background0_h;
		border: 2px solid $green_dark;
		border-radius: 5px;
		margin-bottom: 8px;
	}
	.header_button button:hover {
		cursor: pointer;
		background-color: $orange_light0;
		color: $blue_dark2;
	}
	.header_button button:active {
		background-color: $orange_light;
		color: $blue_dark2;
	}
	// .header_button select {
	// 	color: $red_dark;
	// 	background: $background0_h;
	// }
	.debug {
		margin-top: 40px;
	}
	.debug:after {
		content: '';
		display: table;
		clear: both;
	}
	.column1 {
		float: left;
		width: 30%;
	}
	.column2 {
		// float: left;
		width: 70%;
	}
	.tab_contents {
		display: flex;
		flex-wrap: wrap;
		min-width: 50%;
		width: fit-content;
		box-shadow: 5px 5px 0px 2px $blue_dark2; //, 8px 8px 8px $blue_dark2;
		// filter: blur(2px);
		border: 3px solid $text;
		border-radius: 0px 10px 10px 10px;
		// -webkit-border-radius: 0px 10px 10px 10px;
		// -moz-border-radius: 0px 10px 10px 10px;
		padding: 10px 10px 20px 10px;
	}

	.layout {
		flex: 1 1 auto;
	}
	.options {
		margin: 1rem;
		margin-left: 3rem;
	}
	.options h2 {
		margin-bottom: 5px;
	}
	.options input:not([type='range']) {
		width: 6rem;
	}
	.options_inline {
		flex: 0 0 auto;
	}
	.options_block {
		flex: 0 0 100%;
	}
	.options th {
		text-align: left;
		// padding-top: 1rem;
	}
	.options input {
		margin-top: 6px;
		margin-left: 10px;
	}
	.options input[type="range"] {
		accent-color: $blue_light;
	}
	// these hide show are the contents
	.tabhide {
		display: none;
	}
	.tabshow {
		display: block;
	}
	// these are the tabs themselves
	.tabs {
		margin-top: 20px;
		margin-bottom: -3px;
		margin-left: 3px;
	}
	.tab {
		// border-bottom-color: rgba(0, 0, 0, 0);
		padding-top: 4px;
	}
	.tab:focus {
		outline: none;
	}
	.tab:hover {
		cursor: pointer;
		background: $background0_h;
	}
	.active_tab {
		background: $background1;
		border: 3px solid $active_border;
		border-radius: 5px 5px 0px 0px;
		border-bottom-color: rgba(0, 0, 0, 0);
		border-right: none;
		border-left: none;
		box-shadow:
			3px 0 $active_border,
			-3px 0px $active_border;
	}
	.inactive_tab {
		background: $background2;
		border: 3px solid $text;
		border-radius: 5px 5px 0px 0px;
		border-bottom-color: rgba(0, 0, 0, 0.5);
		border-right: none;
		border-left: none;
		box-shadow:
			3px 0 $text,
			-3px 0px $text;
	}

	.x_button {
		display: inline-block;
		font-size: 14px;
		width: 18px;
		height: 22px;
	}

	.switch {
		position: relative;
		display: inline-block;
		width: 32px;
		height: 18px;
		vertical-align: bottom;
		margin-bottom: 3px;
	}
	/* Hide default HTML checkbox */
	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	/* The slider */
	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: #ccc;
		-webkit-transition: 0.4s;
		transition: 0.4s;
	}

	.slider:before {
		position: absolute;
		content: '';
		height: 12px;
		width: 12px;
		left: 3px;
		bottom: 3px;
		background-color: white;
		-webkit-transition: 0.4s;
		transition: 0.4s;
	}

	input:checked + .slider {
		background-color: $blue_light;
	}

	input:focus + .slider {
		box-shadow: 0 0 1px $blue_light;
	}

	input:checked + .slider:before {
		-webkit-transform: translateX(14px);
		-ms-transform: translateX(14px);
		transform: translateX(14px);
	}

	.slider.round {
		border-radius: 4px;
	}

	.slider.round:before {
		border-radius: 2px;
	}

	.help_doc {
		margin: 2rem;
	}

	.tooltip_div {
		cursor: pointer;
	}
	:global(.tooltip) {
		--tooltip-font-family: Ubuntu Mono, monospace;
		--tooltip-font-size: 20px;
		--tooltip-color: #f2e5bc;
		--tooltip-background-color: #076678; // $foreground0;
	}

	pre {
		// background: black;
		padding: 20px;
		padding-right: 28px;
		border: 2px solid $foreground0;
		box-shadow: inset 4px 4px $blue_dark2, inset -4px -4px $blue_dark2;
		border-radius: 5px;
		background: $background2;
		color: $blue_dark2;
	}
</style>
