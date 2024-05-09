<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount, onDestroy } from 'svelte'
	import { open, ask } from '@tauri-apps/api/dialog';
	import { setContext } from "svelte";
	import Layout from "./Layout.svelte";
	import EffortLayer from "./EffortLayer.svelte";
	import PhalanxLayer from './PhalanxLayer.svelte';
	import { appWindow } from '@tauri-apps/api/window'
	import { layer_switch_regex } from "./utils"


	/**@type {keyof TabComponentMap}*/
	let active_tab = "tab1";
	
	/**
	 * @typedef {Object} TabComponentMap
	 * @property {typeof Layout} tab1
	 * @property {typeof PhalanxLayer} tab3
	 * @property {typeof EffortLayer} tab2
	 
	 */
	/** @type {TabComponentMap} */
	const tab_components = {
		tab1: Layout,
		tab2: EffortLayer,
		tab3: PhalanxLayer,
	}

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

	/**@param {boolean} show_dialog*/
	async function open_toml(show_dialog) {
		/**@type {string | string[] | null}*/
		let opened_file;
		if (show_dialog) {
			opened_file = await open({
				multiple: false,
				filters: [{
					name: 'toml',
					extensions: ['toml']
				}]
			});
		} else {
			opened_file = selected_toml_file
		}
		if (opened_file !== null) {
			invoke('process_config', {configFile: opened_file})
				.then((res) => {
					is_size_from_config = true
					selected_toml_file = opened_file
					selected_size = [res.layout_info.num_rows, res.layout_info.num_cols]
					selected_num_layers = (res.layout_info.layout.match(/Layer/g) || []).length;
					layout_string = res.layout_info.layout;
					effort_layer_string = res.layout_info.effort_layer
					phalanx_layer_string = res.layout_info.phalanx_layer
					console.log(`loaded phalanx layer string ${phalanx_layer_string}`)
					num_threads = res.layout_optimizer_config.num_threads
					genetic_options = res.layout_optimizer_config.genetic_options
					keycode_options = res.layout_optimizer_config.keycode_options
					recompute_valid_keycodes()
					dataset_options = res.layout_optimizer_config.dataset_options
					score_options = res.layout_optimizer_config.score_options
					// is_size_from_config = false
					console.log(`successfully loaded ${opened_file}`)
				})
				.catch((e) => {
					alert(e);
					console.error(e);
				})
			// console.log(selected_file)
			// emit("selected_toml_changed", toml_file)
		}
	}

	/** @type {[number, number][]}*/
	let layout_sizes = [];
	/** @type {[number, number]}*/
	let selected_size; //= [2, 4];
	/** @type {boolean}*/
	let is_size_from_config = false
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
		this.keycode = keycode
		this.locked = locked
		this.symmetric = symmetric
	}
	Key.prototype.toString = function() {
		return `${this.keycode}_${+!this.locked}${+this.symmetric}`
	}

	/** @type {Key[][][]}*/
	let layout;
	/** @type {number[][]}*/
	let effort_layer;
	/** @type {[string, string][][]}*/
	let phalanx_layer;
	

	/** @type {string[]}*/
	let keycodes = [];
	
	async function get_sizes() {
		layout_sizes = await invoke('get_layout_presets')
		// selected_size = layout_sizes[0]
	}
	async function get_keycodes() {
		keycodes = await invoke('get_all_keycodes')
		keycodes = keycodes.filter(x => x != "LS" && x != "LST")
	}
	$: {
		selected_num_layers
		keycodes = keycodes.filter(str => {
			const regex = /^LS\d+$/;
			return !regex.test(str);
		});
		for (let n = 0; n < selected_num_layers; n++) {
			keycodes.push(`LS${n}`)
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
		let output = "   "
		for (let k = 0; k < c; k++) {
			if (is_key) {
				output += k.toString().padStart(7)
			} else {
				output += k.toString().padStart(4)
			}
			output += " "
		}
		return output
	}

	/**
	 * @param {number[][] | [string, string][][]} layer
	*/
	function layer_to_string(layer) {
		let output = ""
		let col_inds = col_indexes_to_string(layer[0].length, false)
		output += col_inds
		output += "\n"
		for (let i = 0; i < layer.length; i++) {
			output += `${i}|`
			for (let j = 0; j < layer[0].length; j++) {
				let v = layer[i][j]
				if (Array.isArray(v)) {
					output += `${v[0][0]}:${v[1][0]}`.padStart(4)
				} else {
					output += v.toString().padStart(4)
				}
			}
			output += " \n"
		}
		return output
	}

	/**
	 * @param {Key[][]} layer
	 * @param {number} layer_num
	*/
	function keycode_layer_to_string(layer, layer_num) {
		let output = ""
		let col_inds = col_indexes_to_string(layer[0].length, true)
		output += col_inds
		output += "\n"
		for (let i = 0; i < layer.length; i++) {
			output += `${i}|`
			for (let j = 0; j < layer[0].length; j++) {
				let v = layer[i][j]
				let keycode_string = v.keycode
				if (layer_switch_regex.test(v.keycode)) {
					const regex = /^LS(\d+)$/;
					const match = v.keycode.match(regex);
					let corresponding_layer = -1;
					if (match) {
						corresponding_layer = parseInt(match[1], 10);
						// we have a layer switch, but we are in the target layer for that layer switch. So, we should replace it with _
						if (layer_num == corresponding_layer) {
							keycode_string = "_"
						}
					} else {
						console.error(`problem converting the X in LSX to a number ${v.keycode}, probably a developer parsing error`)
					}
				}
				output += `${v.keycode}_${+!v.locked}${+v.symmetric}`.padStart(8)
			}
			output += " \n"
		}
		return output
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
		let output = ""
		for (let n = 0; n < layout.length; n++) {
			output += `___Layer ${n}___\n`
			let layer = keycode_layer_to_string(layout[n], n)
			output += layer
		}
		return output
	}

	let saved = false;
	/**
	* @typedef {Object} LayoutInfo
	* @property {number} num_rows
	* @property {number} num_cols
	* @property {string} layout
	* @property {string} effort_layer
	* @property {string} phalanx_layer
	*/
	async function write_toml() {
		saved = true
		/**@type LayoutInfo*/
		let li = {
			num_rows: selected_size[0],
			num_cols: selected_size[1],
			layout: layout_to_string(layout),
			effort_layer: layer_to_string(effort_layer),
			phalanx_layer: layer_to_string(phalanx_layer)
		}
		try {
			await invoke('write_toml', {filename: `${config_dir}/saved.toml`, numThreads: num_threads, layoutInfo: li, geneticOptions: genetic_options, keycodeOptions: keycode_options, datasetOptions: dataset_options, scoreOptions: score_options})
			
		} catch (e) {
			alert(e)
		}
	}


	
	/**
	 * @param {[number, number]} size
	 * @param {string} test*/
	async function create_blank_layers(size, test) {
		if (size && !is_size_from_config) {
			console.log(`creating blank layers ${test}`)
			await invoke('create_blank_layers', {r: size[0], c: size[1], loc: test}).then((res) => {
				effort_layer_string = res[0]
				phalanx_layer_string = res[1]
			}).catch((e) => {
				alert(e)
				console.error(e)
			})
		}
	}
	$: {
		create_blank_layers(selected_size, "from $: selected_size, ...")
	}
	

	async function get_default_genetic_options() {
		await invoke("get_default_genetic_options").then((res) => {
			genetic_options = res
		}).catch((e) => {
			alert(e)
			console.error(e)
		})
	}
	async function get_default_keycode_options() {
		await invoke("get_default_keycode_options").then((res) => {
			keycode_options = res[0]
			valid_keycodes = res[1]
		}).catch((e) => {
			alert(e)
			console.error(e)
		})
	}
	async function recompute_valid_keycodes() {
		await invoke("recompute_valid_keycodes", {options: keycode_options}).then((res) => {
			valid_keycodes = res
		}).catch((e) => {
			alert(e)
			console.error(e)
		})
	}
	async function get_default_dataset_options() {
		await invoke("get_default_dataset_options").then((res) => {
			dataset_options = res
		}).catch((e) => {
			alert(e)
			console.error(e)
		})
	}
	async function get_default_score_options() {
		await invoke("get_default_score_options").then((res) => {
			score_options = res
		}).catch((e) => {
			alert(e)
			console.error(e)
		})
	}


	let options_display = "options_inline"

	function readjust_tab_contents() {
		/**@type {HTMLElement | null}*/
		let layout_element = document.querySelector(".layout")
		/**@type {HTMLElement | null}*/
		let options_inline_element = document.querySelector("options_inline")
		/**@type {HTMLElement | null}*/
		let options_block_element = document.querySelector("options_block")
		if (layout_element !== null && options_inline_element !== null && options_block_element !== null) { 
			if (layout_element.offsetWidth >= options_inline_element.offsetWidth) {
				options_display = "options_block"
			} else {
				options_display = "options_inline"
			}
		}
	}

	let config_dir = ""
	/**@type {number}*/
	let save_interval_timer;
	onMount(() => {

		invoke("get_config_dir").then((res) => {
			config_dir = res
			selected_toml_file = `${config_dir}/saved.toml`
			invoke("does_file_exist", {filename: selected_toml_file}).then((res) => {
				if (res) {
					open_toml(false).then((res) => {
						console.log(JSON.stringify(layout))
						console.log(`size from config ${is_size_from_config}`)
					})
				}
			})
			
		})
		get_sizes().then((res) => {
			selected_size = layout_sizes[0]
		})
		get_keycodes()
		get_default_genetic_options()
		get_default_keycode_options()
		get_default_dataset_options()
		get_default_score_options()


 	 	save_interval_timer = setInterval(write_toml, 10000);
		// appWindow.once("ready", async () => {
		// 	await create_blank_layers("app window ready")
			// selected_size = layout_sizes[0]
		// })
	})
	onDestroy(() => clearInterval(save_interval_timer));
	
	
	let container;
	/**@param {any} event*/
	function handleChange(event) {
		const target = event.target;
		let value;
		if (target) {
			switch (target.type) {
				case "checkbox":
					value = target.checked
					break
				case "range":
					value = target.value
					break
				default:
					console.log(target.type)
			}
			saved = false
		}
	}

</script>

<div class="debug">
<div class="column1">
	<h1>Debug section</h1>
	<p>size: {selected_size}</p>
	<p>file: {selected_toml_file}</p>
	<p>config dir: {config_dir}</p>
	<p>tab: {active_tab}</p>
</div>
<div class="column2">
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
			<td>{col.join(":")}</td>
			{/each}
		</tr>	
		{/each}
	</table>
	{/if}
</div>
</div>

<p>Config dir: <input type="text" bind:value={config_dir} />
<button on:click={write_toml}>Write</button></p>
<p>saved status: {saved}</p>

<h1>Layout section</h1>
<div>
	<button on:click={() => open_toml(true)}>Load config</button>
	or
	choose layout size:
	<!-- {#await get_sizes then} -->
	<!-- bind:value={selected_size} -->
	<select on:change={() => {is_size_from_config = false}} bind:value={selected_size} on:change={readjust_tab_contents}>
	
		{#each layout_sizes as size}
			<option value={size}>{size[0]} x {size[1]}</option>
		{/each}
	</select>
	<!-- {/await} -->
	and number of layers:
	<select bind:value={selected_num_layers}>
		{#each {length: max_layers} as _, i}
			<option value={i+1}>{i+1}</option>
		{/each}
	</select>
</div>


<br>
<br>

{#if selected_size}
<div class="tabs">
	<button on:click={() => active_tab = 'tab1'} class="{active_tab == "tab1" ? "active_tab" : "inactive_tab"} tab">Layout</button>
	<button on:click={() => active_tab = 'tab2'} class="{active_tab == "tab2" ? "active_tab" : "inactive_tab"} tab">Effort layer</button>
	<button on:click={() => active_tab = 'tab3'} class="{active_tab == "tab3" ? "active_tab" : "inactive_tab"} tab">Hand assignment</button>
</div>
<div bind:this={container} on:input={handleChange} on:change={handleChange}>
<div class="tab_contents">
	<div class="layout">
	<div class={active_tab == "tab1" ? "tabshow" : "tabhide"}>
		<svelte:component this={tab_components["tab1"]} bind:layout={layout} bind:keycodes={keycodes} num_layers={selected_num_layers} layout_size={selected_size} layout_string={layout_string} {is_size_from_config} bind:saved={saved}/>
	</div>
	<div class={active_tab == "tab2" ? "tabshow" : "tabhide"}>
		<svelte:component this={tab_components["tab2"]} bind:effort_layer_string={effort_layer_string} bind:effort_layer={effort_layer} bind:layout_size={selected_size} {is_size_from_config} bind:saved={saved} />
	</div>
	<div class={active_tab == "tab3" ? "tabshow" : "tabhide"}>
		<svelte:component this={tab_components["tab3"]} bind:phalanx_layer_string={phalanx_layer_string} bind:phalanx_layer={phalanx_layer} bind:layout_size={selected_size} {is_size_from_config} bind:saved={saved} />
	</div>	
	</div>

	
	
	<div class="options {options_display}">
	<h2>Options</h2>
	num_threads: <input type="range" min=1 max=24 bind:value={num_threads} /> {num_threads}
	{#if genetic_options}
	<h3>Genetic options</h3>
		{#each Object.entries(genetic_options) as [key, value]}
		{#if key == "population_size"}
			{key}: <input type="range" min=0 max=10000 step=100 bind:value={genetic_options[key]} /> <input type="number" bind:value={genetic_options[key]} />
			<br>
		{:else}
			<span>{key} = {value}</span> <br>
		{/if}
		{/each}
	{/if}

	{#if keycode_options}
	<h3>Keycode options</h3>
	{#each Object.entries(keycode_options) as [key, value]}
		{#if key != "explicit_inclusions"}
			{key}: <input type="checkbox" bind:checked={keycode_options[key]} on:change={recompute_valid_keycodes} /> <br>
		{:else}
			<span>{key} = {keycode_options[key]}</span> <br>
		{/if}
	{/each}
	<div style="width: 400px; word-wrap: normal;">Keycode list: <span>{valid_keycodes.join(", ")}</span></div>
	{/if}

	{#if dataset_options}
	<h3>Dataset options</h3>
	{#each Object.entries(dataset_options) as [key, value]}
	{#if key == "dataset_paths"}
		<div style="width: 400px; word-wrap: normal;"><span>{key} = {value.join(", ")}</span></div>
	{:else}
		<span>{key} = {value}</span> <br>
	{/if}
	{/each}
	{/if}

	{#if score_options}
	<h3>Scoring options</h3>
	{#each Object.entries(score_options) as [key, value]}
		<span>{key} = {value}</span> <br>
	{/each}
	{/if}
	</div>
	</div>
</div>
{/if}



<style lang="scss">
	@use "../styles/colors.scss" as *;
	:global(*) {
		color: $text;
	}
	.debug:after {
		content: "";
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
		padding: 10px 10px 10px 10px;
	}
	
	.layout {
		flex: 1 1 auto;
	}
	.options {
		margin: 1rem;
		margin-left: 3rem;
	}
	.options_inline {
		flex: 0 0 auto;
	}
	.options_block {
		flex: 0 0 100%;
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
		box-shadow: 3px 0 $active_border, -3px 0px $active_border;
	}
	.inactive_tab {
		background: $background2;
		border: 3px solid $text;
		border-radius: 5px 5px 0px 0px;
		border-bottom-color: rgba(0, 0, 0, 0.5);
		border-right: none;
		border-left: none;
		box-shadow: 3px 0 $text, -3px 0px $text;
	}

</style>
