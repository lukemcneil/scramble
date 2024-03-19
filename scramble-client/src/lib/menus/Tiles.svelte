<script lang="ts">
	import Tile from './Tile.svelte';

	export let current_letters: Array<string>;
	export let letter_order: Array<number> | null = null;
	export let background_color: string = '#E3CFAA';
	export let show_score: boolean = true;
</script>

<div style="display: flex; flex-wrap: wrap; justify-content: center; padding-bottom: 10px">
	{#if letter_order}
		{#each letter_order as letter_index}
			<Tile letter={current_letters[letter_index]} {background_color} {show_score}></Tile>
		{/each}
	{:else}
		{#each current_letters as letter}
			<Tile {letter} {background_color} {show_score}></Tile>
		{/each}
	{/if}
</div>
