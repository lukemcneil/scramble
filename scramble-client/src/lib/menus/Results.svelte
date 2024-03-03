<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { Answer } from '$lib/datatypes/answer';
	import { onMount } from 'svelte';
	import { getGame, getScore } from '$lib/functions/requests';
	import Tiles from './Tiles.svelte';
	import { tileScores } from './tileScores';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let current_letters: Array<string> = [];
	let answers: Array<Answer> = [];
	let correct_answer_map: Map<string, string> = new Map();
	let my_answer: string;
	let score_map: Map<string, number> = new Map();

	function onNextRoundClick() {
		setGameState('answer');
	}

	function getScores() {
		getScore(game_name)
			.then((response) => response.json())
			.then((data) => {
				for (var prop in data) {
					score_map.set(prop, data[prop]);
				}
				score_map = new Map([...score_map.entries()].sort((a, b) => b[1] - a[1]));
			});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				current_letters = data.rounds[data.rounds.length - 2].letters;
				answers = data.rounds[data.rounds.length - 2].answers;

				answers.forEach((answer: Answer) => {
					correct_answer_map.set(answer.player, answer.answer);
				});
				answers = answers.sort((a1, a2) => wordScore(a2.answer) - wordScore(a1.answer));
				my_answer = correct_answer_map.get(name);
			});
	}

	onMount(() => {
		readGame();
		getScores();
	});

	function wordScore(word: string) {
		let sum = 0;
		word.split('').forEach((c) => {
			sum += tileScores.get(c.toUpperCase());
		});
		return sum;
	}
</script>

<main>
	<h2>Results</h2>
	<Tiles {current_letters}></Tiles>
	<hr />
	<h3>Answers</h3>
	{#each answers as answer}
		<div>
			{answer.player}:
			{wordScore(answer.answer)}
			{#if my_answer}
				<Tiles current_letters={answer.answer.split('')}></Tiles>
			{/if}
		</div>
	{/each}
	<hr />
	<h3>Scores</h3>
	{#each score_map as [player, score]}
		<div>
			{player}: {score}
		</div>
	{/each}
	<div>
		<Button text="Next Round" onClick={onNextRoundClick} />
	</div>
</main>

<style>
	@import '../../app.css';
</style>
