<script>
	import { onMount } from 'svelte'
	import { getContext } from "svelte"
	const notify_symmetric = getContext("symmetric_position")
	const notify_locked = getContext("locked_position")
	const notify_keycode = getContext("keycode_position")

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
		notify_symmetric(current_key_location)
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

	function set_locked() {
		notify_locked(current_key_location)
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

	/**@type {string}*/
	let no_keycode_color;
	/**@type {string}*/
	let no_keycode_border_color;
	let keycode_fade = "";
	$: {
		if (keycode == "NO") {
			no_keycode_color = "#cbb899"; // "#D7C7AC";
			no_keycode_border_color = "#e8dcd0";
			keycode_fade = "keycode_div"
		} else {
			no_keycode_color = "";
			no_keycode_border_color = "";
			keycode_fade = ""
		}
	}

	let keycode_select_class = "keycode_select_nonfaded"

	// $: keycode, notify_keycode(current_key_location, keycode)
	$: {
		// keycode = keycode_item.value
		// if (keycode == "NO") {
		// 	keycode_select_class = "keycode_select_faded"
		// } else {
		// 	keycode_select_class = "keycode_select_nonfaded"
		// }
		notify_keycode(current_key_location, keycode, false)
	}
	function set_keycode_from_select() {
		notify_keycode(current_key_location, keycode, true)
	}

	onMount(() => {
		
	})
	
	
</script>

<style lang="scss">
	@use "../styles/colors.scss" as *;
	:global(*) {
		color: $text;
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
	.keycode_div {
		position: absolute;
		height: 32px;
		width: 66px;
		background: rgba(0, 0, 0, 0.2);
		pointer-events: none;
		border-radius: 8px;
	}
</style>

<div class="key" style="background-color: {no_keycode_color}; border-color: {no_keycode_border_color};">

	<div class="{keycode_fade}"></div>
	<select bind:value={keycode} on:change={set_keycode_from_select}>
		{#each keycodes as keycode}
			<option value={keycode}>{keycode == "NO" ? "" : keycode}</option>
		{/each}
	</select>
	
	
	<!-- <div class="keycode_div">
	<Select {items} bind:value={keycode_item} clearable={false} showChevron={true} class="{keycode_select_class} keycode_class" />
	</div> -->

	<div class="key_flags">
		<button on:click={set_locked} style="background-color: {locked_color};">
			{#if locked}
			🔒
			{:else}
			🔓
			{/if}
		</button>
		
		<button on:click={set_symmetric} disabled={current_key_location[2] == (num_cols - 1) / 2} style="background-color: {symmetric_flag_color}; font-size: 12px;">
			{#if symmetric}
			o|o
			{:else if current_key_location[2] == (num_cols - 1) / 2}
			|
			{:else if current_key_location[2] > (num_cols - 1) / 2}
			|o
			{:else}
			o|
			{/if}
		</button>
		
	</div>
</div>