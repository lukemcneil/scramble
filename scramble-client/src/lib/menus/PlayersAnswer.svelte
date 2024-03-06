<script lang="ts">
	import type { Answer } from '$lib/datatypes/answer';
	import Tiles from './Tiles.svelte';

	export let answer: Answer;
</script>

<div>
	{answer.player}:
	{answer.score}

	{#if answer.score == 0}
		(not a word)
	{:else}
		<button
			style="padding: 1px 1px;"
			on:click={() => window.alert(answer.answer + ': ' + answer.definition)}>define</button
		>
	{/if}
	<Tiles
		current_letters={answer.answer.split('')}
		background_color={answer.score == 0 ? 'lightcoral' : 'lightgreen'}
	></Tiles>
</div>

<style>
	@import '../../app.css';
</style>
