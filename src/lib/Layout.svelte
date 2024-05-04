<script>
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
						<select bind:value={layout[n][i][j].keycode}>
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
		// height: 32px;
		// width: 32px;
		border: $key_outline solid 3px;
		border-radius: 5px;
		font-size: 16px;
		text-align: center;
		padding: 5px;
	}
	.key_flags {
		padding: 0px;
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

