<script>
	import Greet from '../lib/Greet.svelte'
	import Layout from '../lib/Layout.svelte'
</script>
<style lang="scss">
	@import "../styles/colors.scss";
	:global(body) {
		margin: 100px;
		background-color: $background1;
	}
</style>



<h1>Welcome to SvelteKit</h1>
<Greet />
<Layout />
