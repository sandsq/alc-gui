<script>
	import { split_layer_to_rows, split_row_to_columns } from "./utils.js"
	import { onMount, afterUpdate } from 'svelte'
	import { appWindow } from '@tauri-apps/api/window'

	/**@type {number[][]}*/
	export let effort_layer;
	/**@type {string}*/
	export let effort_layer_string;
	/**@type {[number, number]}*/
	export let layout_size;
	/**@type {boolean}*/
	export let is_size_from_config;
	/**@type {boolean}*/
	export let saved

	function resize_effort_layer() {
		effort_layer = [];
		for (let i = 0; i < layout_size[0]; i++) {
			effort_layer.push([]);
			for (let j = 0; j < layout_size[1]; j++) {
				effort_layer[i].push(0.0);
			}
		}
	}
	$: {
		layout_size
		console.log(`layout_size changed to ${layout_size} for resize_effort_layer`)
		if (!is_size_from_config) {
			resize_effort_layer()
		}
	}

	/**@param {string} effort_string*/
	function fill_effort_layer_from_string(effort_string) {
		resize_effort_layer()
		let rows = split_layer_to_rows(effort_string)
		if (effort_layer.length != rows.length) {
			alert(`the number of rows in the effort layer (${effort_layer[0].length}) does not match the number of rows found from the config (${rows.length}: ${rows}); this is probably a developer error due to parsing the config incorrectly`)
			return;
		}
		for (let i = 0; i < rows.length; i++) {
			let row = rows[i]
			let cols = split_row_to_columns(row)
			if (effort_layer[0].length != cols.length) {
				alert(`the number of columns in the effort layer (${effort_layer[0].length}) does not match the number of columns found from the config (${cols.length}: ${cols}); this is probably a developer error due to parsing the config incorrectly`)
				return;
			}
			for (let j = 0; j < cols.length; j++) {
				let col = cols[j]
				effort_layer[i][j] = parseFloat(col)
			}
		}
		console.log(`filled effort layer from string`)
		// console.log(`filled effort layer with ${effort_string}`)
		// console.log(`effort layer ${effort_layer}`)
	}
	
	$: {
		console.log(`effort layer string changed ${effort_layer_string}`)
		if (effort_layer_string) {
			fill_effort_layer_from_string(effort_layer_string)
		}
	}

	// onMount(() => {
	// 	resize_effort_layer()
	// })
	// afterUpdate(() => {
	// 	saved = false
	// });
</script>

{#if effort_layer}
<h2>Effort layer</h2>
<table>
	{#each {length: effort_layer.length} as _, i}
	<tr>
		{#if i == 0}
			<th class="column_indexes"></th>
			{#each {length: effort_layer[0].length} as _, j}
				<th class="column_indexes">{j}</th>
			{/each}
		{/if}
	</tr>
	<tr>
		<th>{i}&nbsp;</th>
		{#each {length: effort_layer[0].length} as _, j}
		<td>
			<div class="effort_key">
			<input type="number" step="0.5" bind:value={effort_layer[i][j]} min=0 />
			</div>
		</td>
		{/each}
	</tr>
	{/each}
</table>
{/if}

<style lang="scss">
	@use "../styles/colors.scss" as *;
	// table {
	// 	border-spacing: 5px;
	// 	margin-bottom: 20px;
	// }
	.effort_key input {
		
		width: $key_dimension;
		height: $key_dimension;
		background: $background0_h;
		text-align: center;
		border: 3px solid $background2;
		border-radius: 5px;
		font-size: 24px;
		text-align: center;
		padding: $key_padding;
		padding-right: $key_padding + 2px;
		box-shadow: $key_shadow_offset $key_shadow_offset $blue_light;
	}
	.column_indexes {
		padding-bottom: 5px;
	}
</style>
