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
		background-color: $background;
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

