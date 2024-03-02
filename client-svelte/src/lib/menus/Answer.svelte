<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { Player } from '$lib/datatypes/player';
	import { onMount } from 'svelte';
	import { getGame, postAnswer, postChangeQuestion, postChatGptQuestion } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let players: Array<Player> = [];
	let current_question: string | undefined = '';
	let round_count: number;

	let answer: string = '';
	let prompt: string = '';

	function onSubmitClick() {
		if (answer == '') {
			console.log('you need a non-empty answer');
			return;
		}
		const response: Promise<Response> = postAnswer(game_name, name, answer);
		response.then((response) => {
			if (response.ok) {
				setGameState('answer_wait');
			}
		});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				current_question = data.rounds[data.rounds.length - 1].question;
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

	function onChangeQuestion() {
		const response: Promise<Response> = postChangeQuestion(game_name);
		readGame();
	}

	function onMrGptQuestion() {
		if (prompt == '') {
			return;
		}
		const response: Promise<Response> = postChatGptQuestion(game_name, prompt);
		readGame();
	}
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<div>
		<Button text="Change Question" onClick={onChangeQuestion} />
	</div>
	<div>
		{current_question}
	</div>
	<div>
		<InputField bind:value={answer} text="enter your answer" />
	</div>
	<div>
		<Button text="Submit" onClick={onSubmitClick} />
	</div>

	<div>Players:</div>
	{#each players as player}
		<div>
			{player}
		</div>
	{/each}
	<div>
		<InputField bind:value={prompt} text="enter Mr. GPT prompt" />
	</div>
	<div>
		<Button text="Get Mr. GPT question" onClick={onMrGptQuestion} />
	</div>
</main>

<style>
	@import '../../app.css';
</style>
