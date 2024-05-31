<script>
	import { split_layer_to_rows, split_row_to_columns, layer_switch_regex } from './utils.js';
	import { onMount, afterUpdate, beforeUpdate } from 'svelte';

	/**
	 * @constructor
	 * @param {string} keycode
	 * @param {boolean} locked
	 * @param {boolean} symmetric
	 */
	function Key(keycode, locked, symmetric) {
		this.keycode = keycode;
		this.locked = locked;
		this.symmetric = symmetric;
	}
	Key.prototype.toString = function () {
		return `${this.keycode}_${+!this.locked}${+this.symmetric}`;
	};

	/**@type {number}*/
	export let num_layers;
	/**@type {[number, number]}*/
	export let layout_size;
	/**@type {Key[][][]}*/
	export let layout;
	export let keycodes;
	export let layout_string;
	/**@type boolean*/
	export let is_size_from_config;
	/**@type boolean*/
	export let saved;
	/**@type boolean*/
	export let hide_flags = false;

	/**
	 * @param {string} keycode
	 * @param {boolean} locked
	 * @param {boolean} symmetric
	 * @returns {Key}
	 */
	function createKey(keycode, locked, symmetric) {
		/**@type {Key}*/
		const key = {
			keycode,
			locked,
			symmetric
		};
		return key;
	}

	/**@param {string} str
	 * @param {string} from
	 * @returns {number | null}
	 */
	function get_target_layer_from_string(str, from) {
		const regex = /^LS(\d+)$/;
		const match = str.match(regex);
		let corresponding_layer = -1;
		if (match) {
			corresponding_layer = parseInt(match[1], 10);
			return corresponding_layer;
		} else {
			let msg = `${str} has no target layer (i.e., the X in LSX). From ${from} This is probably a developer error due to parsing.`;
			alert(msg);
			console.error(msg);
			return null;
		}
	}

	/**@param {string} test*/
	function resize_layout(test) {
		// console.log(`trying to resize layout with ${num_layers} layers and ${layout_size} size from ${test}`)
		if (num_layers && layout_size) {
			layout = [];
			for (let n = 0; n < num_layers; n++) {
				layout.push([]);
				for (let i = 0; i < layout_size[0]; i++) {
					layout[n].push([]);
					for (let j = 0; j < layout_size[1]; j++) {
						let k = createKey('NO', false, false);
						layout[n][i].push(k);
					}
				}
			}
			// console.log(`resized layout${num_layers} layers and ${layout_size} size from ${test}`)
			// console.log(`resized layout, it has ${layout.length} layers and (${layout[0].length} x ${layout[0][0].length}) size`)
		}
	}
	function resize_num_layers() {
		console.log(`trying to adjust num layers to ${num_layers}`);
		if (num_layers) {
			/**@type {Key[][][]}*/
			let output = [];
			for (let n = 0; n < Math.min(layout.length, num_layers); n++) {
				for (let i = 0; i < layout[n].length; i++) {
					for (let j = 0; j < layout[n][i].length; j++) {
						let k = layout[n][i][j].keycode;
						if (layer_switch_regex.test(k) && num_layers < layout.length) {
							const regex = /^LS(\d+)$/;
							const match = k.match(regex);
							let corresponding_layer = -1;
							if (match) {
								corresponding_layer = parseInt(match[1], 10);
								layout[n][i][j].keycode = 'NO';
								layout[corresponding_layer][i][j].keycode = 'NO';
							} else {
								let msg = `${k} has no target layer (i.e., the X in LSX). This is probably a developer error due to parsing.`;
								alert(msg);
								console.error(msg);
							}
						}
					}
				}
				output.push(layout[n]);
			}
			// for (let n = 0; n < Math.min(layout.length, num_layers); n++) {
			// 	output.push(layout[n])
			// }
			while (output.length < num_layers) {
				/**@type {Key[][]}*/
				let layer = [];
				for (let i = 0; i < layout_size[0]; i++) {
					layer.push([]);
					for (let j = 0; j < layout_size[1]; j++) {
						let k = createKey('NO', false, false);
						layer[i].push(k);
					}
				}
				output.push(layer);
			}
			layout = output;
			console.log(`adjusted num layers ${num_layers}`);
		}
	}
	$: {
		layout_size;
		console.log(`layout_size changed to ${layout_size} for resize_layout`);
		if (!is_size_from_config) {
			resize_layout('$: layout_size,...');
		}
	}
	$: num_layers, resize_num_layers();

	/**@param {string} layout_string*/
	function fill_layout_from_string(layout_string) {
		resize_layout('fill_layout_from_string');
		// console.log(`trying to fill layout with ${layout_string}`)
		let layers = layout_string.split(/___(.*)___/g);
		layers = layers.filter((x) => x != '' && !x.includes('Layer'));
		if (layout.length != layers.length) {
			alert(
				`the number of layers in the layout (${layout.length}) does not match the number of layers found from the config (${layers.length}); this is probably a developer error due to parsing the config incorrectly`
			);
			console.log(layers);
			return;
		}
		for (let n = 0; n < layers.length; n++) {
			let layer = layers[n];
			let rows = split_layer_to_rows(layer);
			if (layout[n].length != rows.length) {
				alert(
					`the number of rows in the ${n}th layer of the layout (${layout[n].length}) does not match the number of rows found from the config (${rows.length}: ${rows}); this is probably a developer error due to parsing the config incorrectly`
				);
				return;
			}
			for (let i = 0; i < rows.length; i++) {
				let row = rows[i];
				let cols = split_row_to_columns(row);
				if (layout[n][i].length != cols.length) {
					alert(
						`the number of columns in the ${n}th layer, ${i}ith row of the layout (${layout[n][i].length}) does not match the number of columns found from the config (${cols.length}: ${cols}); this is probably a developer error due to parsing the config incorrectly`
					);
					return;
				}
				for (let j = 0; j < cols.length; j++) {
					let col = cols[j];
					// if the keycode at the current location is LS, skip it since it will have been placed there by its corresponding LS
					if (layer_switch_regex.test(layout[n][i][j].keycode)) {
						continue;
					}
					if (col[0] == '_') {
						layout[n][i][j].keycode = 'NO';
						// gui tracks whether key is locked, backend tracks wether key is moveable (i.e., unlocked), so use extra negate
						layout[n][i][j].locked = !!!parseInt(col[2]);
						layout[n][i][j].symmetric = !!parseInt(col[3]);
					} else {
						let s = col.split('_');
						let k = s[0];
						let flags = s[1];
						if (layer_switch_regex.test(k)) {
							let target_layer = get_target_layer_from_string(
								k,
								'filling layout from string, trying to place corresponding LS key'
							);
							if (target_layer) {
								// let target_layer = parseInt(k[2])
								layout[target_layer][i][j].keycode = k;
								layout[target_layer][i][j].locked = !!!parseInt(flags[0]);
								layout[target_layer][i][j].symmetric = !!parseInt(flags[1]);
							}
						}
						layout[n][i][j].keycode = k;
						layout[n][i][j].locked = !parseInt(flags[0]);
						layout[n][i][j].symmetric = !!parseInt(flags[1]);
					}
				}
			}
		}
		console.log(`filled layout from string`);
	}
	$: {
		if (layout_string) {
			fill_layout_from_string(layout_string);
		}
	}

	let previous_keycode = '';
	/**
	 * @param {number} n
	 * @param {number} i
	 * @param {number} j
	 */
	function set_previous_keycode(n, i, j) {
		previous_keycode = layout[n][i][j].keycode;
	}

	/**
	 * @param {[number, number, number]} pos
	 * @param {boolean} from_select
	 */
	function set_keycode(pos, from_select) {
		let layer = pos[0];
		let row = pos[1];
		let col = pos[2];
		let new_keycode = layout[layer][row][col].keycode;
		console.log(`current keycode at beginning of set_keycode ${previous_keycode}`);
		console.log(`new keycode ${new_keycode}`);
		// if LS is being replaced, replace its corresponding place as well
		if (layer_switch_regex.test(previous_keycode)) {
			// corresponding location might be in a lower layer so check all
			for (let n = 0; n < layout.length; n++) {
				if (layer == n) {
					continue;
				}
				if (layer_switch_regex.test(layout[n][row][col].keycode)) {
					layout[n][row][col].keycode = 'NO';
				}
			}
		}
		if (layer_switch_regex.test(new_keycode)) {
			const regex = /^LS(\d+)$/;
			const match = new_keycode.match(regex);
			let corresponding_layer = -1;
			if (match) {
				corresponding_layer = parseInt(match[1], 10);
				if (layout[corresponding_layer][row][col].locked) {
					alert(
						"can't assign corresponding layer switch to this position as it is occupied by a locked key"
					);
					layout[layer][row][col].keycode = previous_keycode;
				} else if (layout[corresponding_layer][row][col].symmetric) {
					alert(
						"can't assign corresponding layer switch to this position as it is occupied by a symmetric key"
					);
					layout[layer][row][col].keycode = previous_keycode;
				} else {
					layout[corresponding_layer][row][col].keycode = new_keycode;
					// layout[layer][row][col].keycode = previous_keycode
				}
			} else {
				let msg = `${new_keycode} has no target layer (i.e., the X in LSX). From trying to use set_keycode. This is probably a developer error due to parsing.`;
				alert(msg);
				console.error(msg);
			}
		}
		// // if the key is being replaced with LS, specify the corresponding layer
		// if (new_keycode == "LS" && from_select) {
		// 	let corresponding_layer_string = prompt(`specify corresponding layer (${0}-${layout.length - 1}):`)
		// 	if (corresponding_layer_string && Number.isInteger(parseInt(corresponding_layer_string))) {
		// 		let corresponding_layer = parseInt(corresponding_layer_string)
		// 		if (layout[corresponding_layer][row][col].locked) {
		// 			alert("can't assign corresponding layer switch to this position as it is occupied by a locked key")
		// 			layout[layer][row][col].keycode = previous_keycode
		// 		} else if (layout[corresponding_layer][row][col].symmetric) {
		// 			alert("can't assign corresponding layer switch to this position as it is occupied by a symmetric key")
		// 			layout[layer][row][col].keycode = previous_keycode
		// 		} else {
		// 			layout[corresponding_layer][row][col].keycode = "LS"
		// 			// layout[layer][row][col].keycode = previous_keycode
		// 		}
		// 	}
		// }
	}

	/**
	 * @param {number} n
	 * @param {number} i
	 * @param {number} j
	 */
	function set_corresponding_symmetries(n, i, j) {
		let value_to_set = !layout[n][i][j].symmetric;
		layout[n][i][j].symmetric = value_to_set;
		let total_cols = layout[0][0].length;
		let symmetric_col = total_cols - 1 - j;
		layout[n][i][symmetric_col].symmetric = value_to_set;
	}

	let locked_color = '#b37575';
	let symmetric_color = '#99ebc2';

	onMount(() => {
		// resize_layout("from mount")
	});

	// afterUpdate(() => {
	// 	console.log("change happened on layout")
	// 	saved = false
	// });
</script>

{#if layout}
	{#each { length: layout.length } as _, n}
		<h2>Layer {n}</h2>
		<table>
			{#each { length: layout[0].length } as _, i}
				{@const num_cols = layout[0][0].length}
				<tr>
					{#if i == 0}
						<th class="column_indexes"></th>
						{#each { length: layout[0][0].length } as _, j}
							<th class="column_indexes">{j}</th>
						{/each}
					{/if}
				</tr>
				<tr>
					<th>{i}&nbsp;</th>
					{#each { length: num_cols } as _, j}
						<td>
							<div
								class="key 
									{layout[n][i][j].keycode == 'NO' ? 'no_keycode' : 'has_keycode'} 
									{layout[n][i][j].locked ? 'key_locked' : 'key_unlocked'} 
									{layout[n][i][j].symmetric ? 'key_symmetric' : 'key_asymmetric'} 
									{hide_flags ? "hide_flags" : "show_flags"}
									"
							>
								{#if layout[n][i][j].keycode == 'NO'}
									<div class="
										keycode_fade
										{hide_flags ? "keycode_fade_hide_flags" : "keycode_fade_show_flags"}
										"
									></div>
								{/if}

								<select
									on:change={() => set_previous_keycode(n, i, j)}
									bind:value={layout[n][i][j].keycode}
									on:change={() => set_keycode([n, i, j], true)}
								>
									{#each keycodes as keycode}
										<option value={keycode}>{keycode == 'NO' ? '' : keycode}</option>
									{/each}
									</select>
								{#if !hide_flags}
									<span class="key_flag {layout[n][i][j].locked ? 'locked' : 'unlocked'}">
										<button
											on:click={() => {
												layout[n][i][j].locked = !layout[n][i][j].locked;
												saved = false;
											}}
										>
											{#if layout[n][i][j].locked}
												🔒
											{:else}
												🔓
											{/if}
										</button>
									</span>
									<span class="key_flag {layout[n][i][j].symmetric ? 'symmetric' : 'asymmetric'}">
										<button
											on:click={() => {
												set_corresponding_symmetries(n, i, j);
												saved = false;
											}}
											disabled={j == (num_cols - 1) / 2}
										>
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
								{/if}
							</div>
						</td>
					{/each}
				</tr>
			{/each}
		</table>
	{/each}
{/if}

<style lang="scss">
	@use '../styles/colors.scss' as *;
	:global(*) {
		color: $text;
	}
	// table {
	// 	border-spacing: 4px;
	// 	margin-bottom: 20px;
	// }
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
		padding-right: $key_padding + 2px;
		box-shadow: $key_shadow_offset $key_shadow_offset $blue_dark2;
	}
	.hide_flags select {
		height: $key_dimension;
		// width: $key_dimension;
		// margin-right: -2px;
		// margin-top: 1px;
		// font-size: 14px;
	}
	.no_keycode {
		background-color: $background3;
		border: 3px solid $background5;
	}
	.has_keycode {
		background-color: $background1;
		border: 3px solid $background3;
		// box-shadow: 4px 4px $blue_dark1;
	}
	.has_keycode select {
		// background-color: $background;
		border: 4px solid $blue_light;
		margin-left: -2px;
		margin-top: -1px;
	}
	.key_locked {
		background-color: $yellow_light;
	}
	.key_symmetric {
		background-color: $aqua_light;
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
		background-color: $yellow_dark;
	}
	.locked button:hover {
		background-color: $yellow_light;
	}
	.symmetric button {
		background-color: $aqua_dark;
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
		width: 66px;
		background: rgba(0, 0, 0, 0.2);
		pointer-events: none;
		border-radius: 8px;
	}
	.keycode_fade_show_flags {
		height: $key_dimension / 2; //32px;
	}
	.keycode_fade_hide_flags {
		height: $key_dimension;
	}
</style>
