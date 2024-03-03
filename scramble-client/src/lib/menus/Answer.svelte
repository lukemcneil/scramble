<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { Player } from '$lib/datatypes/player';
	import { onMount } from 'svelte';
	import { getGame, postAnswer } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import Tiles from './Tiles.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let players: Array<Player> = [];
	let current_letters: Array<string> = [];
	let round_count: number;
	let error_message: String = '';

	let answer: string = '';
	// let prompt: string = '';

	function onSubmitClick() {
		const response: Promise<Response> = postAnswer(game_name, name, answer);
		response.then((response) => {
			if (response.ok) {
				setGameState('answer_wait');
			} else {
				response.json().then((data) => {
					error_message = data.message;
				});
			}
		});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				current_letters = data.rounds[data.rounds.length - 1].letters;
				round_count = data.rounds.length;
			});
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

	// function onChangeQuestion() {
	// 	const response: Promise<Response> = postChangeQuestion(game_name);
	// 	readGame();
	// }

	// function onMrGptQuestion() {
	// 	if (prompt == '') {
	// 		return;
	// 	}
	// 	const response: Promise<Response> = postChatGptQuestion(game_name, prompt);
	// 	readGame();
	// }
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<Tiles {current_letters}></Tiles>
	<div style="padding-top: 50px">
		<InputField bind:value={answer} text="enter your answer" />
	</div>
	<div>{error_message}</div>
	<div style="padding-bottom: 50px">
		<Button text="Submit" onClick={onSubmitClick} />
	</div>

	{#if round_count == 1}
		<hr />
		<h3>Players:</h3>
		{#each players as player}
			<div>
				{player}
			</div>
		{/each}
	{/if}
	<!-- <div>
		<InputField bind:value={prompt} text="enter Mr. GPT prompt" />
	</div> -->
	<!-- <div>
		<Button text="Get Mr. GPT question" onClick={onMrGptQuestion} />
	</div> -->
</main>

<style>
	@import '../../app.css';
</style>
