<script lang="ts">
	import { onMount } from 'svelte';
	import { getGame } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import Tiles from './Tiles.svelte';
	import type { Answer } from '$lib/datatypes/answer';
	import PlayersAnswer from './PlayersAnswer.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let players: Array<string> = [];
	let current_letters: string | undefined = '';
	let round_count: number;
	let waiting_for: Array<string> = [];
	let answers: Array<Answer> = [];
	let correct_answer_map: Map<string, Answer> = new Map();
	let my_answer: Answer;

	async function readGame() {
		let response = await getGame(game_name);
		let data = await response.json();
		if (response.ok) {
			players = data.players;
			current_letters = data.rounds[data.rounds.length - 1].letters;
			round_count = data.rounds.length;
			if (data.rounds[data.rounds.length - 1].answers.length == 0) {
				setGameState('results');
			} else {
				waiting_for = players.filter(
					(player) =>
						!data.rounds[data.rounds.length - 1].answers.some((answer) => answer.player === player)
				);
				answers = data.rounds[data.rounds.length - 1].answers;
				answers.forEach((answer: Answer) => {
					correct_answer_map.set(answer.player, answer);
				});
				answers = answers.sort((a1, a2) => a2.score - a1.score);
				my_answer = correct_answer_map.get(name);
			}
		} else {
			if (data.error == 'GameNotFound') {
				setGameState('join');
			}
		}
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'answer_wait') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<Tiles {current_letters}></Tiles>
	<hr />
	<h3>Your Answer</h3>
	{#if my_answer}
		<PlayersAnswer answer={my_answer}></PlayersAnswer>
	{/if}
	<hr />
	<h3>Waiting on players...</h3>
	{#each waiting_for as player}
		<div>
			{player}
		</div>
	{/each}
</main>

<style>
	@import '../../app.css';
</style>
