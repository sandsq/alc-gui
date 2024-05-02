<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte'
	import { open } from '@tauri-apps/api/dialog';

	/** @type {string | string[] | null}*/
	let toml_file;

	async function open_toml() {
		toml_file = await open({
			multiple: false,
			filters: [{
				name: 'Config',
				extensions: ['toml']
			}]
		});
	}

	/** @type {string[][]}*/
	let layout;

	/** @type {[number, number][]}*/
	let layout_sizes = [];
	/** @type {[number, number]}*/
	let selected_size;

	/** @type {string[]}*/
	let keycodes = [];
	/** @type {string}*/
	let selected_keycode;

	// /** @type {string[]} */
	// let layout_sizes = []
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

<button on:click={open_toml}>Open</button>

{#await get_sizes then}
<select bind:value={selected_size}>
	{#each layout_sizes as size}
		<option value={size}>{size[0]} x {size[1]}</option>
	{/each}
</select>
{/await}
<p>size: {selected_size}</p>
<p>file: {toml_file}</p>

{#await get_keycodes then}
{#if selected_size}
<table>
	{#each {length: selected_size[0]} as _, i}
	<tr>
		{#each {length: selected_size[1]} as _, j}
			<td class="key">
				<select bind:value={selected_keycode}>
					{#each keycodes as keycode}
						<option value={keycode}>{keycode}</option>
					{/each}
				</select>
				({i}, {j})
			</td>
		{/each}
	</tr>
	{/each}
</table>
{/if}
{/await}
