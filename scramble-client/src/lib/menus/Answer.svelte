<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { onMount } from 'svelte';
	import { getGame, postAnswer } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import Tiles from './Tiles.svelte';
	import type { Round } from '$lib/datatypes/round';
	import type { Game } from '$lib/datatypes/game';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let players: Array<string> = [];
	let current_letters: Array<string> = [];
	let letter_order: Array<number> = [];
	let round_count: number;
	let error_message: String = '';
	let waiting_for: Array<string> = [];
	let guesses_used: number = 0;
	let guesses_allowed: number = 2;
	let show_score: boolean = true;

	let answer: string = '';

	function onSubmitClick() {
		const response: Promise<Response> = postAnswer(game_name, name, answer.trim());
		response.then((response) => {
			if (response.ok) {
				setGameState('answer_wait');
			} else {
				response.json().then((data) => {
					error_message = data.message;
					readGame().then(() => 1);
				});
			}
		});
	}

	async function readGame() {
		let response = await getGame(game_name);
		let data = await response.json();
		if (response.ok) {
			let current_round: Round = data.rounds[data.rounds.length - 1];
			players = data.players;
			current_letters = current_round.letters;
			if (letter_order.length == 0) {
				letter_order = new Array(current_letters.length).fill(null).map((_, i) => i);
			}
			round_count = data.rounds.length;
			waiting_for = players.filter(
				(player) => !current_round.answers.some((answer) => answer.player === player)
			);
			if (name && current_round.guesses_used.hasOwnProperty(name)) {
				guesses_used = current_round.guesses_used[name];
			}
			guesses_allowed = data.settings.number_of_guesses;
			show_score = data.settings.scoring_method == 'Normal';
		} else {
			if (data.error == 'GameNotFound') {
				setGameState('join');
			}
		}
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'answer') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});

	function shuffle_array<T>(array: T[]): T[] {
		let currentIndex = array.length,
			randomIndex;
		// While there remain elements to shuffle.
		while (currentIndex != 0) {
			// Pick a remaining element.
			randomIndex = Math.floor(Math.random() * currentIndex);
			currentIndex--;
			// And swap it with the current element.
			[array[currentIndex], array[randomIndex]] = [array[randomIndex], array[currentIndex]];
		}
		return array;
	}

	function shuffle_tiles() {
		letter_order = shuffle_array(letter_order);
	}
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<Tiles {current_letters} {letter_order} {show_score}></Tiles>
	<div>
		<Button text="Shuffle" onClick={shuffle_tiles} />
	</div>
	<div style="padding-top: 30px">
		<InputField bind:value={answer} text="enter your answer" />
	</div>
	<div>{error_message}</div>
	<div>Guesses left: {guesses_allowed - guesses_used}</div>
	<div style="padding-bottom: 50px">
		<Button text="Submit" onClick={onSubmitClick} />
	</div>

	<hr />
	<h3>Waiting on players...</h3>
	{#each waiting_for as player}
		<div>
			{player}
		</div>
	{/each}

	{#if round_count == 1}
		<hr />
		<h3>Players:</h3>
		{#each players as player}
			<div>
				{player}
			</div>
		{/each}
	{/if}
</main>

<style>
	@import '../../app.css';
</style>
