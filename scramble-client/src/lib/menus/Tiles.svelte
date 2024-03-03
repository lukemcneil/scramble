<script lang="ts">
	import Tile from './Tile.svelte';

	export let current_letters: Array<string>;
</script>

<div style="display: flex; justify-content: center; padding-bottom: 10px">
	{#each current_letters as letter}
		<Tile {letter}></Tile>
	{/each}
</div>
