<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { Answer } from '$lib/datatypes/answer';
	import { onMount } from 'svelte';
	import { getGame, getScore } from '$lib/functions/requests';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let letters: string;
	let answers: Array<Answer> = [];
	let correct_answer_map: Map<string, string> = new Map();
	let my_answer: string;
	let my_guess: Array<Answer> = [];
	let my_guess_map: Map<string, string> = new Map();
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
				console.log(score_map);
			});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				letters = data.rounds[data.rounds.length - 2].letters;
				answers = data.rounds[data.rounds.length - 2].answers;

				answers.forEach((answer: Answer) => {
					correct_answer_map.set(answer.player, answer.answer);
				});
				my_answer = correct_answer_map.get(name);
			});
	}

	onMount(() => {
		readGame();
		getScores();
	});
</script>

<main>
	<h2>Results</h2>
	<div>
		{letters}
	</div>
	<div>
		You said: {my_answer}
	</div>
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
