<script lang="ts">
	import Tile from './Tile.svelte';

	export let current_letters: Array<string>;
	export let letter_order: Array<number> | null = null;
</script>

<div style="display: flex; justify-content: center; padding-bottom: 10px">
	{#if letter_order}
		{#each letter_order as letter_index}
			<Tile letter={current_letters[letter_index]}></Tile>
		{/each}
	{:else}
		{#each current_letters as letter}
			<Tile {letter}></Tile>
		{/each}
	{/if}
</div>
