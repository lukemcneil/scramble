<!-- Tile.svelte -->
<script lang="ts">
	import { tileScores } from './tileScores';

	export let letter;
</script>

<div class="tile">
	<span>{letter.toUpperCase()}</span>
	<span class="score">{tileScores.get(letter.toUpperCase())}</span>
</div>

<style>
	.tile {
		width: 50px;
		height: 50px;
		border: 2px solid #333; /* Default border color */
		display: flex;
		justify-content: center;
		align-items: center;
		font-size: 24px; /* Bigger font size for the letter */
		font-weight: bold;
		background-color: #f9f9f9; /* Default background color */
		color: #333; /* Default text color */
		border-radius: 5px;
		position: relative;
		margin: 0 1px; /* Add margin to the left and right */
	}

	.score {
		position: absolute;
		bottom: 5px;
		right: 5px;
		font-size: 14px;
		color: #333; /* Default score color */
	}
</style>
