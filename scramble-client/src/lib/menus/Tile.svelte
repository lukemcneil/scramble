<!-- Tile.svelte -->
<script lang="ts">
	export let letter;

	const tileScores: Map<string, number> = new Map([
		['E', 1],
		['A', 1],
		['I', 1],
		['O', 1],
		['N', 1],
		['R', 1],
		['T', 1],
		['L', 1],
		['S', 1],
		['U', 1],
		['D', 2],
		['G', 2],
		['B', 3],
		['C', 3],
		['M', 3],
		['P', 3],
		['F', 4],
		['H', 4],
		['V', 4],
		['W', 4],
		['Y', 4],
		['K', 5],
		['J', 8],
		['X', 8],
		['Q', 10],
		['Z', 10]
	]);
</script>

<div class="tile">
	<span>{letter.toUpperCase()}</span>
	<span class="score">{tileScores.get(letter.toUpperCase())}</span>
</div>

<style>
	.tile {
		width: 50px;
		height: 50px;
		border: 2px solid #333;
		display: flex;
		justify-content: center;
		align-items: center;
		font-size: 24px; /* Bigger font size for the letter */
		font-weight: bold;
		background-color: #f9f9f9;
		border-radius: 5px;
		cursor: pointer;
		position: relative;
	}

	.score {
		position: absolute;
		bottom: 5px;
		right: 5px;
		font-size: 12px;
		color: #666;
	}
</style>
