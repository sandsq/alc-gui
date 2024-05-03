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
</script>

<style lang="scss">
	@use "../styles/colors.scss" as *;
	.key {
		background-color: $key_background;
		// height: 32px;
		// width: 32px;
		border: $key_outline solid 4px;
		border-radius: 4px;
		font-size: 16px;
		text-align: center;
	}
	.key_flags {
		padding: 0px;
	}
	.key_flags button {
		padding: 0px;
		font-size: 16px;
		width: 24px;
		height: 24px;
	}
	select {
		font-size: 16px;
	}
</style>

<div class="key">
	<select bind:value={keycode}>
		{#each keycodes as keycode}
			<option value={keycode}>{keycode}</option>
		{/each}
	</select>
	<div class="key_flags">
		<button on:click={() => locked = !locked}>
			{#if locked}
			🔒
			{:else}
			🔓
			{/if}
		</button>
		<button on:click={set_symmetric} disabled={current_key_location[2] == (num_cols - 1) / 2}>
			{#if symmetric}
			S
			{:else}
			A
			{/if}
		</button>
	</div>
</div>