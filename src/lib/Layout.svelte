<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'

	/** @type {[number, number][]}*/
	let layout_sizes = [];

	/** @type {[number, number]}*/
	let selected;

	// /** @type {string[]} */
	// let layout_sizes = []
	async function get_sizes() {
		layout_sizes = await invoke('get_layout_presets')
		selected = layout_sizes[0]
	}
	
	

	onMount(() => {
		get_sizes()
	})

</script>

<style lang="scss">
	@import "../styles/colors.scss";
	.key {
		background-color: $key_background;
		height: 48px;
		width: 48px;
	}
</style>

{#await get_sizes}
{:then}
<select bind:value={selected}>
	{#each layout_sizes as size}
		<option value={size}>{size[0]} x {size[1]}</option>
	{/each}
</select>
{/await}


<!-- <table>
	{#each {length: num_rows} as _, i}
	<tr>
		{#each {length: num_cols} as _, j}
			<td class="key">
				({i}, {j})
			</td>
		{/each}
	</tr>
	{/each}
</table> -->