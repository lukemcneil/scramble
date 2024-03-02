<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { Player } from '$lib/datatypes/player';
	import { onMount } from 'svelte';
	import { getGame, postAnswer } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;

	let players: Array<string> = [];
	let current_question: string | undefined = '';
	let round_count: number;
	let waiting_for: Array<string> = [];

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				current_question = data.rounds[data.rounds.length - 1].question;
				round_count = data.rounds.length;
				console.log(data);
				if (data.rounds[data.rounds.length - 1].answers.length == players.length) {
					setGameState('guess');
				} else {
					waiting_for = players.filter(
						(player) =>
							!data.rounds[data.rounds.length - 1].answers.some(
								(answer) => answer.player === player
							)
					);
					console.log(waiting_for);
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
	<h3>Waiting on players...</h3>
	{#each waiting_for as player}
		<div>
			{player}
		</div>
	{/each}
	<div>
		{current_question}
	</div>
</main>

<style>
	@import '../../app.css';
</style>
