<script>
	import { getContext } from "svelte"
	const notifyParent = getContext("symmetric_position")

	/**@type {string}*/
	export let keycode;
	/**@type {string[]}*/
	export let keycodes;
	/**@type {boolean}*/
	export let locked;
	/**@type {boolean}*/
	export let symmetric;
	/**@type {[number, number, number]}*/
	export let current_key_location;
	/**@type {number}*/
	export let num_cols;

	function set_symmetric() {
		notifyParent(current_key_location)
	}
	/**@type {string}*/
	let symmetric_flag_color;
	$: {
		if (symmetric) {
			symmetric_flag_color = "#99ebc2";
		} else {
			symmetric_flag_color = "";
		}
	}
	/**@type {string}*/
	let locked_color;
	$: {
		if (locked) {
			locked_color = "#b37575";
		} else {
			locked_color = "";
		}
	}
</script>

<style lang="scss">
	@use "../styles/colors.scss" as *;
	:global(*) {
		color: $text;
	}
	.key {
		background-color: $key_background;
		// height: 32px;
		// width: 32px;
		border: $key_outline solid 2px;
		border-radius: 4px;
		font-size: 16px;
		text-align: center;
		padding: 3px;
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
		background-color: $key_background;
	}
	select {
		font-size: 16px;
		font-weight: bold;
		border: 2px solid $border;
	}
</style>

<div class="key">
	<select bind:value={keycode}>
		{#each keycodes as keycode}
			<option value={keycode}>{keycode}</option>
		{/each}
	</select>
	<div class="key_flags">
		<button on:click={() => locked = !locked} style="background-color: {locked_color};">
			{#if locked}
			🔒
			{:else}
			🔓
			{/if}
		</button>
		
		<button on:click={set_symmetric} disabled={current_key_location[2] == (num_cols - 1) / 2} style="background-color: {symmetric_flag_color}; font-size: 12px;">
			{#if symmetric}
			o|o
			{:else if current_key_location[2] > (num_cols - 1) / 2}
			|o
			{:else}
			o|
			{/if}
		</button>
		
	</div>
</div>