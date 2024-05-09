<script>
	import { split_layer_to_rows, split_row_to_columns } from "./utils.js"
	import { onMount } from 'svelte'

	/**@type {[string, string][][]}*/
	export let phalanx_layer;
	/**@type {string}*/
	export let phalanx_layer_string;
	/**@type {[number, number]}*/
	export let layout_size;
	/**@type {boolean}*/
	export let is_size_from_config;


	let hands = ["left", "right"]
	let fingers = ["thumb", "index", "middle", "ring", "pinkie", "joint"]

	function resize_phalanx_layer() {
		phalanx_layer = [];
		for (let i = 0; i < layout_size[0]; i++) {
			phalanx_layer.push([]);
			for (let j = 0; j < layout_size[1]; j++) {
				phalanx_layer[i].push(["left", "index"]);
			}
		}
	}
	$: {
		layout_size
		// if (!is_size_from_config) {
		resize_phalanx_layer()
		// }
	}

	/**@param {string} phalanx_string*/
	function fill_phalanx_layer_from_string(phalanx_string) {
		resize_phalanx_layer()
		console.log(`filling phalanx\n${phalanx_layer_string}`)
		let rows = split_layer_to_rows(phalanx_string)
		if (phalanx_layer.length != rows.length) {
			alert(`the number of rows in the phalanx layer (${phalanx_layer[0].length}) does not match the number of rows found from the config (${rows.length}: ${rows}); this is probably a developer error due to parsing the config incorrectly`)
			return;
		}
		for (let i = 0; i < rows.length; i++) {
			let row = rows[i]
			let cols = split_row_to_columns(row)
			if (phalanx_layer[0].length != cols.length) {
				alert(`the number of columns in the phalanx layer (${phalanx_layer[0].length}) does not match the number of columns found from the config (${cols.length}: ${cols}); this is probably a developer error due to parsing the config incorrectly`)
				return;
			}
			for (let j = 0; j < cols.length; j++) {
				let col = cols[j]
				let ph = col.split(":")
				let hand = ph[0]
				let finger = ph[1]
				switch(hand.toLowerCase()) {
					case "l":
						hand = "left"
						break
					case "r":
						hand = "right"
						break
					default:
				}
				switch(finger.toLowerCase()) {
					case "t":
						finger = "thumb"
						break
					case "i":
						finger = "index"
						break
					case "m":
						finger = "middle"
						break
					case "r":
						finger = "ring"
						break
					case "p":
						finger = "pinkie"
						break
					case "j":
						finger = "joint"
						break
				}
				phalanx_layer[i][j] = [hand, finger]
			}
		}
		
	}
	$: {
		if (phalanx_layer_string) {
			fill_phalanx_layer_from_string(phalanx_layer_string)
		}
		
	}

	// onMount(() => {
	// 	resize_phalanx_layer()
	// })
</script>

{#if phalanx_layer}
<h2>Phalanx layer</h2>
<table>
	{#each {length: phalanx_layer.length} as _, i}
	<tr>
		{#if i == 0}
			<th class="column_indexes"></th>
			{#each {length: phalanx_layer[0].length} as _, j}
				<th class="column_indexes">{j}</th>
			{/each}
		{/if}
	</tr>
	<tr>
		<th>{i}&nbsp;</th>
		{#each {length: phalanx_layer[0].length} as _, j}
		<td>
			<div class="phalanx_key">
				<div class="hand_selectors">
						<select bind:value={phalanx_layer[i][j][0]}>
							{#each hands as hand}
								<option value={hand}>{hand}</option>
							{/each}
						</select>
					
						<select bind:value={phalanx_layer[i][j][1]}>
							{#each fingers as finger}
								<option value={finger}>{finger}</option>
							{/each}
						</select>
				</div>	
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
	.phalanx_key {
		width: $key_dimension;
		height: $key_dimension;
		background: $background1;
		text-align: center;
		border: 3px solid $background3;
		border-radius: 5px;
		font-size: 24px;
		padding: $key_padding;
		padding-right: $key_padding + 2px;
		box-shadow: $key_shadow_offset $key_shadow_offset $blue_dark1;
	}
	.phalanx_key select {
		font-size: 12px;
		font-weight: bold;
	}
	.hand_selectors {
		margin-top: 2px;
	}
	.column_indexes {
		padding-bottom: 5px;
	}
</style>
