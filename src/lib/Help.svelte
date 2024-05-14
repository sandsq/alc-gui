<script>
	import { open as shell_open } from '@tauri-apps/api/shell';
</script>

<h1>Advanced layout calculator</h1>
<p>Please report issues or make suggestions at <a href="/" on:click={async () => shell_open("https://github.com/sandsq/alc-gui")}>https://github.com/sandsq/alc-gui</a>!</p>

<h2>Overview</h2>
<p>This app is designed to optimize a keyboard layout under various configurations.</p>

<p>Each position in the layout is given an "effort value", representing the relative effort required to reach that position. Each value is also paired with a hand+finger assignment, which is used to determine whether typing a given sequence results a favorable action such as hand alternation or finger rolling.</p>

<p>To find an optimized layout, the input is copied and randomized multiple times to form a population of layouts. Each layout in this population is then scored against various text datasets. The best layouts, i.e., those layouts that require the least amount of effort to type the text data, are further duplicated and modified to form a new generation of layouts. This process continues for a set number of generations and the final best 10 layouts are saved.</p>

<h2>The layout</h2>
<p>The `Layout` tab lets the user customize the starting layout. This does not need to be filled out fully, as the optimizer will insert keycodes into blank slots. See <a href="/" on:click={async () => await shell_open("https://docs.qmk.fm/#/keycodes")}>QMK keycodes</a> for keycodes; those that generally are not used in typing text are not included (for now). In this app, use `_NO` as QMK's `KC_TRNS` and use `_NO` with a lock as QMK's `KC_NO`. The user must also take care to ensure that every layer is reachable by using the `LSX` keycodes. For example, `LS1` on layer 0 means pressing that key while layer 0 is active will switch to layer 1. Selecting an `LS` keycode will automatically place a corresponding keycode in the target layer as well. So continuing the previous example, `LS1` will automatically appear in layer 1 in the corresponding location. The optimizer assumes that layers are arranged as recommended in the beginner section of `Working with Layers` from the <a href="/" on:click={async () => await shell_open("https://docs.qmk.fm/#/feature_layers?id=layers")}>QMK documentation</a>. Layer switches in the optimizer are designed to act as one-shot switches or momentary switches, depending on configuration; other layer switch types are currently not supported.</p>

<p>Mod-taps, for home row mods or similar, are currently not supported either -- however, in my experience, the user generally wants to position these themselves.</p>

<p>Tap dance and combos are planned as well.</p>

<p>Keys for which usage data generally cannot be collected, such as backspace, F-keys, navigation, etc., should be manually placed by the user, whether by using their keycode or by locking out positions for them.</p>

<p>There are two additional flags per key. The lock can be toggled to ensure that the specified keycode will be locked to its specific position. This is useful if the user has certain muscle memory, keyboard shortcuts, etc. that they wish to preserve. The symmetry flag can be toggled to ensure that the two specified keys always remain symmetric to each other. The optimizer can still move their positions, but symmetry will be retained. This is useful for modifiers like shift or for various brackets. One feature in the works is constraining keys relative to each other. For example, the user may want to place corresponding brackets adjacent to each other, rather than symmetrically across the board.</p>

<p>Arbitrary layout sizes are not supported for now (because I decided to try out Rust's constant generics to specify array sizes, oops). Instead, choose the next largest layout size and block unused keys using the lock toggle. This approach can also be used to create irregular layout shapes, such as ergo ones. Staggered layouts can still be optimized by mapping each key to a grid position -- there just currently isn't a visual indicator for stagger.</p>

<h2>The effort layer</h2>
<p>The `Effort layer` tab lets the user customize the effort layer. Values represent the relative effort required to reach a certain position -- higher value means more effort needed. As a rule of thumb, give the most accessible locations a value of 1 and scale the remaining positions accordingly. Some experimentation may be required to find numbers that suit the user.</p>

<h2>The hand assignments</h2>
<p>The `Hand assignment` tab lets the user customize hand and finger assignments per key. These are use to determine whether sequences result in hand alternation or finger rolls, two favorable typing actions that can decrease the effort required to type said sequence. `Joint` refers to that part of the palm that meets the base of the pinkie, as some typists use that part of their hand to hit the keys in the bottom left / bottom right corners. Default hand assignments follow the homerow style of typing, but the user can change that to suit their needs.</p>

<h2>The options</h2>
<p>Hover over the `?` mark accompanying each option to read about it.</p>

<h2>Copyable version</h2>
<p>This is a small convenience for using the layout with QMK. For now all it does is prepend `KC_` to the relevant keycodes and add some commas.</p>

<h2>Misc info</h2>
<p>Using the <a href="/" on:click={async () => await shell_open("https://github.com/morhetz/gruvbox")}>gruvbox color scheme</a>.</p>


<style lang="scss">
	@use '../styles/colors.scss' as *;
	a {
		color: $blue_dark1;
	}
	// a:visited {
	// 	color: $aqua_light;
	// }
	
</style>