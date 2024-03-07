<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { Answer } from '$lib/datatypes/answer';
	import { onMount } from 'svelte';
	import { getGame, getScore } from '$lib/functions/requests';
	import Tiles from './Tiles.svelte';
	import type { WordInfo } from '$lib/datatypes/wordInfo';
	import PlayersAnswer from './PlayersAnswer.svelte';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let current_letters: Array<string> = [];
	let answers: Array<Answer> = [];
	let correct_answer_map: Map<string, string> = new Map();
	let my_answer: string;
	let score_map: Map<string, number> = new Map();
	let best_answers: Array<WordInfo> = [];

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
				best_answers = data.rounds[data.rounds.length - 2].best_answers;

				answers.forEach((answer: Answer) => {
					correct_answer_map.set(answer.player, answer.answer);
				});
				answers = answers.sort((a1, a2) => a2.score - a1.score);
				my_answer = correct_answer_map.get(name);
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'results') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
		getScores();
	});
</script>

<main>
	<h2>Results</h2>
	<Tiles {current_letters}></Tiles>
	<hr />
	<h3>Answers</h3>
	{#each answers as answer}
		<PlayersAnswer {answer}></PlayersAnswer>
	{/each}
	<hr />
	<h3>Best Answers</h3>
	{#each best_answers as answer}
		<div>
			<button
				style="padding: 1px 1px;"
				on:click={() => window.alert(answer.word + ': ' + answer.definition)}
				>{answer.word.toLowerCase()}</button
			>: {answer.score}
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
