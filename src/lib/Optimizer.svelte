<script>
	import Key from "./Key.svelte"
	import { split_layer_to_rows, split_row_to_columns } from "./utils.js"
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'
	import { open } from '@tauri-apps/api/dialog';
	import { emit, listen, once } from '@tauri-apps/api/event'
	import { setContext } from "svelte";
	import Layout from "./Layout.svelte";

	/**@type {keyof TabComponentMap}*/
	let active_tab = "tab1";
	
	/**
	 * @typedef {Object} TabComponentMap
	 * @property {typeof Layout} tab1
	 */

	/** @type {TabComponentMap} */
	const tab_components = {
		tab1: Layout,
	}

	/**
	* @typedef Payload
	* @type {object}
	* @property {string} message
	* @property {boolean} pass
	*/
	// async function start_config_error_listener() {
	// 	const unlisten = await once('config-error', (event) => {
	// 		if (event.payload.pass === false) {
	// 			alert(event.payload.message);
	// 			console.log(event.payload.message);
	// 			toml_file = "";
	// 		}
	// 	});
	// }

	/** @type {string | string[] | null}*/
	let selected_toml_file;
	async function open_toml() {
		const opened_file = await open({
			multiple: false,
			filters: [{
				name: 'Config',
				extensions: ['toml']
			}]
		});
		if (opened_file !== null) {
			invoke('process_config', {configFile: opened_file})
				.then((res) => {
					console.log(`successfully loaded ${opened_file}`)
					selected_toml_file = opened_file
					selected_size = [res.layout_info.num_rows, res.layout_info.num_cols]
					selected_num_layers = (res.layout_info.layout.match(/Layer/g) || []).length;
					fill_layout_from_string(res.layout_info.layout);
				})
				.catch((e) => {
					alert(e);
					console.error(e);
				})
			// console.log(selected_file)
			// emit("selected_toml_changed", toml_file)
		}
	}

	/**@param {string} layout_string*/
	function fill_layout_from_string(layout_string) {
		resize_layout()
		let layers = layout_string.split(/___(.*)___/g)
		layers = layers.filter((x) => x != "" && !x.includes("Layer"))
		if (layout.length != layers.length) {
			alert(`the number of layers in the layout (${layout.length}) does not match the number of layers found from the config (${layers.length}) this is probably a developer error due to parsing the config incorrectly`)
			return;
		}
		for (let n = 0; n < layers.length; n++) {
			let layer = layers[n]
			let rows = split_layer_to_rows(layer)
			if (layout[0].length != rows.length) {
				alert(`the number of rows in the layout (${layout[0].length}) does not match the number of rows found from the config (${rows.length}: ${rows}) this is probably a developer error due to parsing the config incorrectly`)
				return;
			}
			for (let i = 0; i < rows.length; i++) {
				let row = rows[i]
				let cols = split_row_to_columns(row)
				if (layout[0][0].length != cols.length) {
					alert(`the number of columns in the layout (${layout[0][0].length}) does not match the number of columns found from the config (${cols.length}: ${cols}) this is probably a developer error due to parsing the config incorrectly`)
					return;
				}
				for (let j = 0; j < cols.length; j++) {
					let col = cols[j]
					console.log(`${col}`)
					if (col[0] == "_") {
						layout[n][i][j].keycode = "NO"
						// gui tracks whether key is locked, backend tracks wether key is moveable (i.e., unlocked), so use extra negate
						layout[n][i][j].locked = !!!parseInt(col[2])
						layout[n][i][j].symmetric = !!parseInt(col[3])
					} else {
						let s = col.split("_")
						let k = s[0]
						let flags = s[1]
						k = k.includes("LS") ? "LS" : k
						layout[n][i][j].keycode = k 
						layout[n][i][j].locked = !!!parseInt(flags[0])
						layout[n][i][j].symmetric = !!parseInt(flags[1])
					}
				}
			}
		}
	}

	/** @type {[number, number][]}*/
	let layout_sizes = [];
	/** @type {[number, number]}*/
	let selected_size = [2, 4];
	/** @type {number}*/
	let selected_num_layers = 3;
	let max_layers = 15;


	/**
	* @typedef {Object} Key
	* @property {string} keycode
	* @property {boolean} locked
	* @property {boolean} symmetric
	*/
	/**
	 * @param {string} keycode
	 * @param {boolean} locked
	 * @param {boolean} symmetric
	 * @returns {Key}
	 */
	function createKey(keycode, locked, symmetric) {
		/**@type {Key}*/
		const key = {
			keycode, locked, symmetric
		};
		return key;
	}
	/** @type {Key[][][]}*/
	let layout;

	/** @type {string[]}*/
	let keycodes = [];
	/** @type {string}*/
	let selected_keycode;
	/** @type {boolean}*/
	let selected_locked_test;

	async function get_sizes() {
		layout_sizes = await invoke('get_layout_presets')
		selected_size = layout_sizes[0]
	}
	async function get_keycodes() {
		keycodes = await invoke('get_all_keycodes')
		selected_keycode = keycodes[0]
	}

	function resize_layout() {
		if (selected_num_layers && selected_size) {	
			layout = [];
			for (let n = 0; n < selected_num_layers; n++) {
				layout.push([]);
				for (let i = 0; i < selected_size[0]; i++) {
					layout[n].push([]);
					for (let j = 0; j < selected_size[1]; j++) {
						let k = createKey("NO", false, false);
						layout[n][i].push(k);
					}
				}
			}
		}
	}
	
	/**
	 * @param {[number, number, number]} pos
	*/
	function set_symmetric(pos) {
		let layer = pos[0]
		let row = pos[1]
		let col = pos[2]
		let value_to_set = !layout[layer][row][col].symmetric
		layout[layer][row][col].symmetric = value_to_set
		let total_cols = layout[0][0].length;
		let symmetric_col = (total_cols - 1) - col;
		layout[layer][row][symmetric_col].symmetric = value_to_set
	}
	setContext("symmetric_position", set_symmetric)
	
	/**
	 * @param {[number, number, number]} pos
	*/
	function set_locked(pos) {
		let layer = pos[0]
		let row = pos[1]
		let col = pos[2]
		let value_to_set = !layout[layer][row][col].locked
		layout[layer][row][col].locked = value_to_set
	}
	setContext("locked_position", set_locked)

	/**
	 * @param {[number, number, number]} pos
	 * @param {string} k
	*/
	function set_keycode(pos, k) {
		let layer = pos[0]
		let row = pos[1]
		let col = pos[2]
		layout[layer][row][col].keycode = k
	}
	setContext("keycode_position", set_keycode)

	onMount(() => {
		get_sizes()
		get_keycodes()
		resize_layout()
		// start_config_error_listener()
	})
</script>

<style lang="scss">
	@use "../styles/colors.scss" as *;
	:global(*) {
		color: $text;
	}
	.column1 {
		float: left;
		width: 30%;
	}
	.column2 {
		float: left;
		width: 70%;
	}
	h1 {
		font-size: 32px;
	}
	// select, button {
	// 	font-size: 16px;
	// }
</style>

<div class="column1">
	<h1>Debug section</h1>
	<p>size: {selected_size}</p>
	<p>file: {selected_toml_file}</p>
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
</div>



<h1>Layout section</h1>
<div>
	<button on:click={open_toml}>Load config</button>
	or
	choose layout size:
	{#await get_sizes then}
	<select bind:value={selected_size} on:change={resize_layout}> 
		{#each layout_sizes as size}
			<option value={size}>{size[0]} x {size[1]}</option>
		{/each}
	</select>
	{/await}
	and number of layers:
	<select bind:value={selected_num_layers} on:change={resize_layout}>
		{#each {length: max_layers} as _, i}
			<option value={i+1}>{i+1}</option>
		{/each}
	</select>	
</div>

<br>
<br>

<div>
 	<button on:click={() => active_tab = 'tab1'}>Layout</button>
	<!-- <button on:click={() => active_tab = 'tab2'}>Effort layer</button> -->
</div>
	  
<svelte:component this={tab_components[active_tab]} {...{layout: layout, keycodes: keycodes}} />
	
<!-- <Layout /> -->




<!-- {#if layout}
{#each {length: layout.length} as _, n}
	<table>
		{#each {length: layout[0].length} as _, i}
		{@const num_cols = layout[0][0].length}
		<tr>
			{#if i == 0}
				<th class="column_indexes"></th>
				{#each {length: layout[0][0].length} as _, j}
					<th class="column_indexes">{j}</th>
				{/each}
			{/if}
		</tr>
		<tr>
			<th>{i}&nbsp;</th>
			{#each {length: num_cols} as _, j}
				<td>
					<Key bind:keycode={layout[n][i][j].keycode} keycodes={keycodes} bind:locked={layout[n][i][j].locked} bind:symmetric={layout[n][i][j].symmetric} current_key_location={[n, i, j]} num_cols={num_cols} />
				</td>
			{/each}
		</tr>
		{/each}
	</table>
{/each}
{/if} -->

