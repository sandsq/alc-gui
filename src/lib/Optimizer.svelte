<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'
	import { open } from '@tauri-apps/api/dialog';
	import { setContext } from "svelte";
	import Layout from "./Layout.svelte";
	import EffortLayer from "./EffortLayer.svelte";

	/**@type {keyof TabComponentMap}*/
	let active_tab = "tab1";
	
	/**
	 * @typedef {Object} TabComponentMap
	 * @property {typeof Layout} tab1
	 * @property {typeof EffortLayer} tab2
	 */
	/** @type {TabComponentMap} */
	const tab_components = {
		tab1: Layout,
		tab2: EffortLayer,
	}

	/** @type {string | string[] | null}*/
	let selected_toml_file;
	/**@type {string}*/
	let layout_string;
	/**@type {string}*/
	let effort_layer_string;

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
					effort_layer_string = res.layout_info.effort_layer
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
	/** @type {number[][]}*/
	let effort_layer;

	/** @type {string[]}*/
	let keycodes = [];
	
	async function get_sizes() {
		layout_sizes = await invoke('get_layout_presets')
		selected_size = layout_sizes[0]
	}
	async function get_keycodes() {
		keycodes = await invoke('get_all_keycodes')
	}

	let active_tab_color = "#b7a78C"
	let active_tab_highlight = "#B85F28"

	onMount(() => {
		get_sizes()
		get_keycodes()
	})
</script>

<div class="debug">
<div class="column1">
	<h1>Debug section</h1>
	<p>size: {selected_size}</p>
	<p>file: {selected_toml_file}</p>
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
 	<button on:click={() => active_tab = 'tab1'} class="tab" style="background: {active_tab == "tab1" ? active_tab_color : ""}; border-bottom: 2px solid {active_tab == "tab1" ? active_tab_highlight : "rgba(0, 0, 0, 0)"};">Layout</button>
	<button on:click={() => active_tab = 'tab2'} class="tab" style="background: {active_tab == "tab2" ? active_tab_color : ""}; border-bottom: 2px solid {active_tab == "tab2" ? active_tab_highlight : "rgba(0, 0, 0, 0)"};">Effort layer</button>
</div>
<div class="tab_area">
	<div class={active_tab == "tab1" ? "tab1show" : "tab1hide"}>
		<svelte:component this={tab_components["tab1"]} bind:layout={layout} bind:keycodes={keycodes} bind:num_layers={selected_num_layers} bind:layout_size={selected_size} bind:layout_string={layout_string} />
	</div>
	<div class={active_tab == "tab2" ? "tab2show" : "tab2hide"}>
		<svelte:component this={tab_components["tab2"]} bind:effort_layer_string={effort_layer_string} bind:effort_layer={effort_layer} bind:layout_size={selected_size} />
	</div>
</div>

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
	.tab_area {
		width: 80%;
		box-shadow: inset 5em 5em "#555555";
		border: 3px solid $text;
		border-radius: 10px;
		padding: 10px 0px 10px 10px;
		// margin: 10px;
	}
	.tab1hide, .tab2hide {
		display: none;
	}
	.tab1show, .tab2show {
		display: block;
	}
	.tab {
		margin-left: 10px;
		border: 2px solid $border;
		// border-radius: 4px 4px 0px 0px;
		border-radius: 10px 4px;
		background: $key_background1;
		border-bottom: none;
	}
</style>
