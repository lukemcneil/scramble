<!-- Tile.svelte -->
<script lang="ts">
	import { tileScores } from './tileScores';

	export let letter;
	export let background_color: string;
</script>

<div class="tile" style="--background_color:{background_color}">
	<span>{letter.toUpperCase()}</span>
	<span class="score">{tileScores.get(letter.toUpperCase())}</span>
</div>

<style>
	.tile {
		width: 40px;
		height: 40px;
		/* border: 0px solid #333; */
		display: flex;
		justify-content: center;
		align-items: center;
		font-size: 24px; /* Bigger font size for the letter */
		font-weight: bold;
		background-color: var(--background_color); /* Default background color */
		color: #333; /* Default text color */
		/* border-radius: 3px; */
		position: relative;
		margin: 1px 1px; /* Add margin to the left and right */
	}

	.score {
		position: absolute;
		bottom: 0px;
		right: 0px;
		font-size: 12px;
		color: #333; /* Default score color */
	}
</style>
