<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { Round } from '$lib/datatypes/round';
	import { Answer } from '$lib/datatypes/answer';
	import { onMount } from 'svelte';
	import { Guess } from '$lib/datatypes/guess';
	import { getGame, postGuess } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import { Picture } from '$lib/datatypes/picture';
	import Matching from '$lib/Matching.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let question: string;
	let players: Array<string> = [];
	let pictures: Array<Picture> = [];
	// let other_players: Array<string> = [];
	let answers: Array<Answer> = [];
	let rounds: Array<Round> = [];
	let guess_player_list: Array<string> = [];
	let guess: Guess = new Guess(name, []);
	let baskets: Array<{ name: string; item: string }> = [];

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				question = data.rounds[data.rounds.length - 1].question;
				if (players.length == 0) {
					players = data.players;
				}
				answers = data.rounds[data.rounds.length - 1].answers;
				pictures = data.rounds[data.rounds.length - 1].pictures;

				if (baskets.length == 0) {
					answers.forEach((answer) => {
						if (answer.player != name) {
							baskets = [...baskets, { name: answer.answer, item: '' }];
						}
					});
				}
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'guess') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});

	function onSubmit() {
		for (let i = 0; i < baskets.length; i++) {
			if (baskets[i].item == '') {
				return;
			}
		}
		baskets.forEach((basket, i) => {
			guess.answers.push(new Answer(basket.item, basket.name));
		});
		const response: Promise<Response> = postGuess(game_name, guess);
		response.then((response) => {
			if (response.ok) {
				setGameState('guess_wait');
			}
		});
	}
</script>

<main>
	<h2>Guess who said what</h2>
	<div>{question}</div>
	<Matching {baskets} players={players.filter((e) => e !== name)} {pictures} />
	<div>
		<Button text="Submit" onClick={onSubmit} />
	</div>
	<div>
		<!-- {#each pictures as pic }
			<div>{pic.player}</div>
			<img src="http://127.0.0.1:8000/{pic.url}" alt="Shut up"/>
		{/each} -->
	</div>
</main>

<style>
	@import '../../app.css';
</style>
