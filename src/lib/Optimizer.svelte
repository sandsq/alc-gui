<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'
	import { open, ask } from '@tauri-apps/api/dialog';
	import { setContext } from "svelte";
	import Layout from "./Layout.svelte";
	import EffortLayer from "./EffortLayer.svelte";
	import PhalanxLayer from './PhalanxLayer.svelte';
	import { appWindow } from '@tauri-apps/api/window'


	/**@type {keyof TabComponentMap}*/
	let active_tab = "tab1";
	
	/**
	 * @typedef {Object} TabComponentMap
	 * @property {typeof Layout} tab1
	 * @property {typeof EffortLayer} tab2
	 * @property {typeof PhalanxLayer} tab3
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
	// /**@type {string}*/
	// let effort_layer_string;
	// /**@type {string}*/
	// let phalanx_layer_string;

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
					phalanx_layer_string = res.layout_info.phalanx_layer
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
	}

	// let active_tab_color = "#FBF1C7"
	// let active_tab_border = "#d65d0e"
	// let inactive_tab_border = "#847b6b"

	/**
	* @typedef {Object} Payload
	* @property {string} message
	* @property {boolean} pass
	*/
	let test_path = "/home/sand/Downloads/test.txt"
	async function write_toml() {
		/**@type Payload*/
		let p = { message: effort_layer_string, pass: true }
		try {
			await invoke('write_toml', {filename: test_path, p: p})
		} catch (e) {
			alert(e)
		}
	}

	/**@type {string}*/
	let effort_layer_string; 
	/**@type {string}*/
	let phalanx_layer_string; 

	/**@param {string} test*/
	async function create_blank_layers(test) {
		console.log(test)
		await invoke('create_blank_layers', {r: selected_size[0], c: selected_size[1]}).then((res) => {
			effort_layer_string = res[0]
			phalanx_layer_string = res[1]
		}).catch((e) => {
			alert(e)
			console.error(e)
		})
	}

	$: {
		selected_size, create_blank_layers("from $: selected_size")
		// invoke('create_blank_layers', {r: selected_size[0], c: selected_size[1]})
		// .then((res) => {
		// 	effort_layer_string = res[0]
		// 	phalanx_layer_string = res[1]
		// })
		// .catch((e) => {
		// 	alert(e)
		// 	console.error(e)
		// })
	}

	onMount(() => {
		get_sizes().then((res) => {
			// if I do this assignment, then the effort layer and hand assignment don't update
			selected_size = layout_sizes[0]
		})
		
		get_keycodes()

		// console.log(`effort layer str ${effort_layer_string} phalanx layer str ${phalanx_layer_string}`)
		// create_blank_layers()
		// appWindow.once("ready", async () => {
		// 	await create_blank_layers("app window ready")
		// 	selected_size = [4, 10]
		// })
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

<input type="text" bind:value={test_path} />
<button on:click={write_toml}>Write</button>

<h1>Layout section</h1>
<div>
	<button on:click={open_toml}>Load config</button>
	or
	choose layout size:
	<!-- {#await get_sizes then} -->
	<select bind:value={selected_size}>
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

<!-- {#if effort_layer_string && phalanx_layer_string} -->
<div class="tabs">
	<button on:click={() => active_tab = 'tab1'} class="{active_tab == "tab1" ? "active_tab" : "inactive_tab"} tab">Layout</button>
	<button on:click={() => active_tab = 'tab2'} class="{active_tab == "tab2" ? "active_tab" : "inactive_tab"} tab">Effort layer</button>
	<button on:click={() => active_tab = 'tab3'} class="{active_tab == "tab3" ? "active_tab" : "inactive_tab"} tab">Hand assignment</button>
</div>
<div class="tab_contents">
	<div class={active_tab == "tab1" ? "tabshow" : "tabhide"}>
		<svelte:component this={tab_components["tab1"]} bind:layout={layout} bind:keycodes={keycodes} bind:num_layers={selected_num_layers} bind:layout_size={selected_size} bind:layout_string={layout_string} />
	</div>
	<div class={active_tab == "tab2" ? "tabshow" : "tabhide"}>
		<svelte:component this={tab_components["tab2"]} bind:effort_layer_string={effort_layer_string} bind:effort_layer={effort_layer} bind:layout_size={selected_size} />
	</div>
	<div class={active_tab == "tab3" ? "tabshow" : "tabhide"}>
		<svelte:component this={tab_components["tab3"]} bind:phalanx_layer_string={phalanx_layer_string} bind:phalanx_layer={phalanx_layer} bind:layout_size={selected_size} />
	</div>
</div>
<!-- {/if} -->

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
	.tab_contents {
		// display: inline-block;
		min-width: 50%;
		width: fit-content;
		box-shadow: 5px 5px $blue_dark2;
		border: 3px solid $text;
		border-radius: 0px 10px 10px 10px;
		padding: 10px 10px 10px 10px;
		// margin: 10px;
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
