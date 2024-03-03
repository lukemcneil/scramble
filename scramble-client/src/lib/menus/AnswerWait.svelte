<script lang="ts">
	import { onMount } from 'svelte';
	import { getGame, postAnswer } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import Tiles from './Tiles.svelte';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;

	let players: Array<string> = [];
	let current_letters: string | undefined = '';
	let round_count: number;
	let waiting_for: Array<string> = [];

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				current_letters = data.rounds[data.rounds.length - 1].letters;
				round_count = data.rounds.length;
				if (data.rounds[data.rounds.length - 1].answers.length == 0) {
					setGameState('results');
				} else {
					waiting_for = players.filter(
						(player) =>
							!data.rounds[data.rounds.length - 1].answers.some(
								(answer) => answer.player === player
							)
					);
				}
			});
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
