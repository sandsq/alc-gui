<script>
	import { split_layer_to_rows, split_row_to_columns } from "./utils.js"
	import { getContext } from "svelte"
	import { onMount } from 'svelte'
	

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
	function resize_num_layers() {
		if (num_layers) {
			/**@type {Key[][][]}*/
			let output = []
			for (let n = 0; n < Math.min(layout.length, num_layers); n++) {
				output.push(layout[n])
			}
			while (output.length < num_layers) {
				/**@type {Key[][]}*/
				let layer = []
				for (let i = 0; i < layout_size[0]; i++) {
					layer.push([]);
					for (let j = 0; j < layout_size[1]; j++) {
						let k = createKey("NO", false, false);
						layer[i].push(k);
					}
				}
				output.push(layer)
			}
			layout = output
		}
	}
	$: layout_size, resize_layout()
	$: num_layers, resize_num_layers()


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
					<div class="keyoffset key {layout[n][i][j].keycode == "NO" ? "no_keycode" : "has_keycode"}">
						{#if layout[n][i][j].keycode == "NO"}
							<div class="keycode_fade"></div>
						{/if}

						<select on:change={() => set_previous_keycode(n, i, j)} bind:value={layout[n][i][j].keycode} on:change={() => set_keycode([n, i, j], true)}>
							{#each keycodes as keycode}
								<option value={keycode}>{keycode == "NO" ? "" : keycode}</option>
							{/each}
						</select>
						
							<span class="key_flag {layout[n][i][j].locked ? "locked" : "unlocked"}">
								<button on:click={() => layout[n][i][j].locked = !layout[n][i][j].locked}>
									{#if layout[n][i][j].locked}
									🔒
									{:else}
									🔓
									{/if}
								</button>
							</span>
							<span class="key_flag {layout[n][i][j].symmetric ? "symmetric" : "asymmetric"}">
								<button on:click={() => set_corresponding_symmetries(n, i, j)} disabled={j == (num_cols - 1) / 2}>
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
							</span>
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
		border-spacing: 5px;
		margin-bottom: 20px;
	}
	.column_indexes {
		padding-bottom: 5px;
	}
	.key {
		margin: 0 auto;
		width: $key_dimension;
		height: $key_dimension;
		border-radius: 5px;
		font-size: 16px;
		text-align: center;
		padding: $key_padding;
		box-shadow: 5px 5px $blue_dark2;
	}
	.keyoffset {
		padding-right: $key_padding + 2px;
	}
	.no_keycode {
		background-color: $background3; 
		border: 3px solid $background5;
	}
	.has_keycode {
		background-color: $background1;
		border: 3px solid $background3;
		box-shadow: 5px 5px $blue_dark1;
	}
	.has_keycode select {
		background-color: $background;
		border: 3px solid $purple_light;
	}
	.key_flag button {
		margin-top: 5px;
		font-size: 12px;
		width: 24px;
		height: 24px;
		font-weight: bold;
		border: 2px solid $border;
		border-radius: 4px;
		background-color: $key_background1;
		padding: 0px;
	}
	.key_flag button:hover {
		cursor: pointer;
		background-color: $background;
	}
	.locked button {
		background-color: $yellow_light;
	}
	.locked button:hover {
		background-color: $yellow_light;
	}
	.unlocked button {
		
	}
	.symmetric button {
		background-color: $aqua_light;
		font-size: 12px;
	}
	.symmetric button:hover {
		background-color: $aqua_light;
	}
	.asymmetric button {
		font-size: 12px;
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

