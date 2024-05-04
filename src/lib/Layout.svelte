<script>
	import { getContext } from "svelte"
	import Key from "./Key.svelte";
	export let layout;
	export let keycodes;
</script>

<style lang="scss">
	table {
		border-spacing: 2px;
		margin-bottom: 20px;
	}
	.column_indexes {
		padding-bottom: 5px;
	}
</style>

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
					<Key bind:keycode={layout[n][i][j].keycode} keycodes={keycodes} bind:locked={layout[n][i][j].locked} bind:symmetric={layout[n][i][j].symmetric} current_key_location={[n, i, j]} num_cols={num_cols} />
				</td>
			{/each}
		</tr>
		{/each}
	</table>
{/each}
{/if}
