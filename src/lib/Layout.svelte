<script>
	import { split_layer_to_rows, split_row_to_columns } from "./utils.js"
	import { getContext } from "svelte"
	import { onMount } from 'svelte'
	/**
	* @typedef {Object} Key
	* @property {string} keycode
	* @property {boolean} locked
	* @property {boolean} symmetric
	*/

	/**@type {number}*/
	export let num_layers;
	/**@type {[number, number]}*/
	export let layout_size;
	/**@type {Key[][][]}*/
	export let layout;
	export let keycodes;
	export let layout_string;

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

	function resize_layout() {
		if (num_layers && layout_size) {	
			layout = [];
			for (let n = 0; n < num_layers; n++) {
				layout.push([]);
				for (let i = 0; i < layout_size[0]; i++) {
					layout[n].push([]);
					for (let j = 0; j < layout_size[1]; j++) {
						let k = createKey("NO", false, false);
						layout[n][i].push(k);
					}
				}
			}
		}
	}
	$: num_layers, resize_layout()
	$: layout_size, resize_layout()

	/**@param {string} layout_string*/
	function fill_layout_from_string(layout_string) {
		resize_layout()
		let layers = layout_string.split(/___(.*)___/g)
		layers = layers.filter((x) => x != "" && !x.includes("Layer"))
		if (layout.length != layers.length) {
			alert(`the number of layers in the layout (${layout.length}) does not match the number of layers found from the config (${layers.length}); this is probably a developer error due to parsing the config incorrectly`)
			console.log(layers)
			return;
		}
		for (let n = 0; n < layers.length; n++) {
			let layer = layers[n]
			let rows = split_layer_to_rows(layer)
			if (layout[0].length != rows.length) {
				alert(`the number of rows in the layout (${layout[0].length}) does not match the number of rows found from the config (${rows.length}: ${rows}); this is probably a developer error due to parsing the config incorrectly`)
				return;
			}
			for (let i = 0; i < rows.length; i++) {
				let row = rows[i]
				let cols = split_row_to_columns(row)
				if (layout[0][0].length != cols.length) {
					alert(`the number of columns in the layout (${layout[0][0].length}) does not match the number of columns found from the config (${cols.length}: ${cols}); this is probably a developer error due to parsing the config incorrectly`)
					return;
				}
				for (let j = 0; j < cols.length; j++) {
					let col = cols[j]
					if (layout[n][i][j].keycode.includes("LS")) {
						continue
					}
					if (col[0] == "_") {
						layout[n][i][j].keycode = "NO"
						// gui tracks whether key is locked, backend tracks wether key is moveable (i.e., unlocked), so use extra negate
						layout[n][i][j].locked = !!!parseInt(col[2])
						layout[n][i][j].symmetric = !!parseInt(col[3])
					} else {
						let s = col.split("_")
						let k = s[0]
						let flags = s[1]
						if (k.includes("LS")) {
							let target_layer = parseInt(k[2])
							layout[target_layer][i][j].keycode = "LS"
							layout[target_layer][i][j].locked = !!!parseInt(flags[0])
							layout[target_layer][i][j].symmetric = !!parseInt(flags[1])
							k = "LS"
						}
						layout[n][i][j].keycode = k 
						layout[n][i][j].locked = !parseInt(flags[0])
						layout[n][i][j].symmetric = !!parseInt(flags[1])
					}
				}
			}
		}
	}
	$: {
		if (layout_string) {
			fill_layout_from_string(layout_string)
		}
		
	}

	let previous_keycode = "";
	/**
	 * @param {number} n
	 * @param {number} i
	 * @param {number} j
	*/
	function set_previous_keycode(n, i, j) {
		previous_keycode = layout[n][i][j].keycode
	}

	/**
	 * @param {[number, number, number]} pos
	 * @param {boolean} from_select
	*/
	function set_keycode(pos, from_select) {
		let layer = pos[0]
		let row = pos[1]
		let col = pos[2]
		let new_keycode = layout[layer][row][col].keycode
		console.log(`current keycode at start ${previous_keycode}`)
		console.log(`new keycode ${new_keycode}`)
		// if LS is being replaced, replace its corresponding place as well
		if (previous_keycode == "LS") {
			for (let n = 0; n < layout.length; n++) {
				if (layer == n) {
					continue
				}
				if (layout[n][row][col].keycode == "LS") {
					layout[n][row][col].keycode = "NO"
				}
			}
		}
		// if the key is being replaced with LS, specify the corresponding layer
		if (new_keycode == "LS" && from_select) {
			let corresponding_layer_string = prompt(`specify corresponding layer (${0}-${layout.length - 1}):`)
			if (corresponding_layer_string && Number.isInteger(parseInt(corresponding_layer_string))) {
				let corresponding_layer = parseInt(corresponding_layer_string)
				if (layout[corresponding_layer][row][col].locked) {
					alert("can't assign corresponding layer switch to this position as it is occupied by a locked key")
					layout[layer][row][col].keycode = previous_keycode
				} else if (layout[corresponding_layer][row][col].symmetric) {
					alert("can't assign corresponding layer switch to this position as it is occupied by a symmetric key")
					layout[layer][row][col].keycode = previous_keycode
				} else {
					layout[corresponding_layer][row][col].keycode = "LS"
					// layout[layer][row][col].keycode = previous_keycode
				}
			}
		}
		
	}
	

	/**
	 * @param {number} n
	 * @param {number} i
	 * @param {number} j
	*/
	function set_corresponding_symmetries(n, i, j) {
		let value_to_set = !layout[n][i][j].symmetric
		layout[n][i][j].symmetric = value_to_set
		let total_cols = layout[0][0].length;
		let symmetric_col = (total_cols - 1) - j;
		layout[n][i][symmetric_col].symmetric = value_to_set
		
	}

	let locked_color = "#b37575"
	let symmetric_color = "#99ebc2"

	let no_keycode_color = "#cbb899";
	let no_keycode_border_color = "#e8dcd0";

	onMount(() => {
		resize_layout()
	})

</script>

{#if layout}
{#each {length: layout.length} as _, n}
	<h2>Layer {n}</h2>
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
					<div class="key" style="background-color: {layout[n][i][j].keycode == "NO" ? no_keycode_color: ""}; border-color: {layout[n][i][j].keycode == "NO" ? no_keycode_border_color: ""};">
						{#if layout[n][i][j].keycode == "NO"}
						<div class="keycode_fade"></div>
						{/if}
						<select on:change={() => set_previous_keycode(n, i, j)} bind:value={layout[n][i][j].keycode} on:change={() => set_keycode([n, i, j], true)}>
							{#each keycodes as keycode}
								<option value={keycode}>{keycode == "NO" ? "" : keycode}</option>
							{/each}
						</select>
						
						<div class="key_flags">
							<button on:click={() => layout[n][i][j].locked = !layout[n][i][j].locked} style="background-color: {layout[n][i][j].locked ? locked_color : ""};">
								{#if layout[n][i][j].locked}
								🔒
								{:else}
								🔓
								{/if}
							</button>
							
							<button on:click={() => set_corresponding_symmetries(n, i, j)} disabled={j == (num_cols - 1) / 2} style="font-size: 12px; background-color: {layout[n][i][j].symmetric ? symmetric_color : ""}">
								{#if layout[n][i][j].symmetric}
								o|o
								{:else if j == (num_cols - 1) / 2}
								|
								{:else if j > (num_cols - 1) / 2}
								|o
								{:else}
								o|
								{/if}
							</button>
							
						</div>
					</div>


				</td>
			{/each}
		</tr>
		{/each}
	</table>
{/each}
{/if}



<style lang="scss">
	@use "../styles/colors.scss" as *;
	:global(*) {
		color: $text;
	}
	table {
		border-spacing: 2px;
		margin-bottom: 20px;
	}
	.column_indexes {
		padding-bottom: 5px;
	}
	.key {
		background-color: $key_background1;
		width: $key_dimension;
		height: $key_dimension;
		border: $key_outline solid 3px;
		border-radius: 5px;
		font-size: 16px;
		text-align: center;
		padding: 5px;
	}
	.key_flags {
		padding: 0px;
		padding-top: 0.2em;
	}
	.key_flags button {
		padding: 0px;
		font-size: 16px;
		width: 24px;
		height: 24px;
		font-weight: bold;
		border: 2px solid $border;
		border-radius: 4px;
		background-color: $key_background1;
	}
	.key_flags button:hover {
		cursor: pointer;
		background-color: $background1;
	}
	select {
		font-size: 18px;
		font-weight: bold;
		border: 2px solid $border;
		border-radius: 8px;
	}
	select:hover {
		cursor: pointer;
	}
	.keycode_fade {
		position: absolute;
		height: 32px;
		width: 66px;
		background: rgba(0, 0, 0, 0.2);
		pointer-events: none;
		border-radius: 8px;
	}
</style>

