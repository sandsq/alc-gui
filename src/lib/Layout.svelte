
<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'
	import { open } from '@tauri-apps/api/dialog';
	import { emit, listen, once } from '@tauri-apps/api/event'

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
		/**
		 * @type {Key}
		 */
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
	 * @param {number} layer
	 * @param {number} row
	 * @param {number} col
	*/
	function set_symmetric(layer, row, col) {
		let value_to_set = !layout[layer][row][col].symmetric
		layout[layer][row][col].symmetric = value_to_set
		let total_cols = layout[0][0].length;
		let symmetric_col = (total_cols - 1) - col;
		layout[layer][row][symmetric_col].symmetric = value_to_set
	}

	onMount(() => {
		get_sizes()
		get_keycodes()
		resize_layout()
		// start_config_error_listener()
	})

	// $: {
	// 	layout = [];
	// 	if (selected_num_layers && selected_size) {	
	// 		for (let n = 0; n < selected_num_layers; n++) {
	// 			layout.push([]);
	// 			for (let i = 0; i < selected_size[0]; i++) {
	// 				layout[n].push([]);
	// 				for (let j = 0; j < selected_size[1]; j++) {
	// 					layout[n][i].push("A");
	// 				}
	// 			}
	// 		}
	// 	}
	// }
</script>

<style lang="scss">
	@use "../styles/colors.scss" as *;
	.key {
		background-color: $key_background;
		height: 32px;
		width: 32px;
		border: $key_outline solid 4px;
		border-radius: 4px;
		font-size: 16px;
		text-align: center;
	}
	.key_flags {
		padding: 0px;
	}
	.key_flags button {
		padding: 0px;
		width: 24px;
		height: 24px;
	}
	.column {
		float: left;
		width: 50%;
	}
	select, button {
		font-size: 16px;
	}
	table {
		border-spacing: 5px;
		margin-bottom: 10px;
	}
	h1 {
		font-size: 32px;
	}
</style>

<div class="column">
	<h1>Debug section</h1>
	<p>size: {selected_size}</p>
	<p>file: {selected_toml_file}</p>
</div>
<div class="column">
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

<!-- {#await get_keycodes then} -->
<!-- {#if selected_size && selected_num_layers} -->
{#if layout}
{#each {length: layout.length} as _, n}
	<table>
		{#each {length: layout[0].length} as _, i}
		{@const num_cols = layout[0][0].length}
		<tr>
			<th></th>
			{#if i == 0}
				{#each {length: layout[0][0].length} as _, j}
					<th>{j}</th>
				{/each}
			{/if}
		</tr>
		<tr>
			<th>{i}</th>
			{#each {length: num_cols} as _, j}
				<td class="key">
					<select bind:value={layout[n][i][j].keycode}>
						{#each keycodes as keycode}
							<option value={keycode}>{keycode}</option>
						{/each}
					</select>
					<div class="key_flags">
						<button on:click={() => layout[n][i][j].locked = !layout[n][i][j].locked}>
							{#if layout[n][i][j].locked}
							🔒
							{:else}
							🔓
							{/if}
						</button>
						<!-- <button on:click={() => layout[n][i][j].symmetric = !layout[n][i][j].symmetric}> -->
						<button on:click={() => set_symmetric(n, i, j)} disabled={j == (num_cols - 1) / 2}>
							{#if layout[n][i][j].symmetric}
							S
							{:else}
							A
							{/if}
						</button>
					</div>
				</td>
			{/each}
		</tr>
		{/each}
	</table>
{/each}
{/if}
<!-- {/await} -->
