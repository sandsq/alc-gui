<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'
	import { open } from '@tauri-apps/api/dialog';
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

	/** @type {string | string[] | null}*/
	let selected_toml_file;
	/**@type {string}*/
	let layout_string;
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
					layout_string = res.layout_info.layout;
					// fill_layout_from_string(res.layout_info.layout);
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

	onMount(() => {
		get_sizes()
		get_keycodes()		
		// start_config_error_listener()
	})
</script>

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
	h1 {
		font-size: 32px;
	}
</style>

<div class="debug">
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
</div>

<h1>Layout section</h1>
<div>
	<button on:click={open_toml}>Load config</button>
	or
	choose layout size:
	{#await get_sizes then}
	<select bind:value={selected_size}> 
		{#each layout_sizes as size}
			<option value={size}>{size[0]} x {size[1]}</option>
		{/each}
	</select>
	{/await}
	and number of layers:
	<select bind:value={selected_num_layers}>
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


<!-- <svelte:component this={tab_components[active_tab]} {...{layout: layout, keycodes: keycodes, num_layers: selected_num_layers, layout_size: selected_size}} /> -->
<svelte:component this={tab_components[active_tab]} bind:layout={layout} bind:keycodes={keycodes} bind:num_layers={selected_num_layers} bind:layout_size={selected_size} bind:layout_string={layout_string} />

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

