<script lang="ts">
	import type { Round } from '$lib/datatypes/round';
	import { Answer } from '$lib/datatypes/answer';
	import { onMount } from 'svelte';
	import { getGame } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;

	let question: string;
	let players: Array<string> = [];
	let waiting_for: Array<string> = [];

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
                question = data.rounds[data.rounds.length - 1].question;
				players = data.players;
                if (data.rounds[data.rounds.length - 1].guesses.length == 0) {
                    setGameState('results');
                } else {
					waiting_for = players.filter(
						(player) => !data.rounds[data.rounds.length - 1].guesses.some((guess) => guess.player === player)
					);
				}
				// players.forEach((player: string) => {
				// 	if (player != name) {
				// 		other_players.push(player);
				// 	}
				// });
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'guess_wait') {
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
	<h2>Guess who said what</h2>
    <h3>Waiting for other players...</h3>
	{#each waiting_for as player}
		<div>
			{player}
		</div>
	{/each}
	<div>{question}</div>
</main>

<style>
	@import '../../app.css';
</style>
