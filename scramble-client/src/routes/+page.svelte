<script lang="ts">
	import { onMount } from 'svelte';
	import Join from '$lib/menus/Join.svelte';
	import Answer from '$lib/menus/Answer.svelte';
	import AnswerWait from '$lib/menus/AnswerWait.svelte';
	import Results from '$lib/menus/Results.svelte';

	import { deleteGame, deletePlayerFromGame, getGame } from '$lib/functions/requests';
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';

	let game_state: string | null;

	let production_url: string = 'https://scramble-vr7r.onrender.com/game/';
	let test_url: string = 'http://0.0.0.0:8172/game/';

	function setGameState(new_state: string) {
		localStorage.setItem('game_state', new_state);
		game_state = new_state;
	}

	onMount(() => {
		if (!localStorage.getItem('game_state')) {
			setGameState('join');
		} else {
			game_state = localStorage.getItem('game_state');
		}
		if (window.location.href == 'http://localhost:5173/') {
			localStorage.setItem('base_server_path', test_url);
		} else {
			localStorage.setItem('base_server_path', production_url);
		}
		if (localStorage.getItem('game_name')) {
			const response: Promise<Response> = getGame(localStorage.getItem('game_name'));
			response.then((response) => {
				if (!response.ok) {
					setGameState('join');
				}
			});
		}
	});

	let player_to_kick: string;

	function onLeave() {
		if (confirm('Do you really want to leave the game?') == true) {
			const response: Promise<Response> = deletePlayerFromGame(
				localStorage.getItem('game_name'),
				localStorage.getItem('name')
			);
			response.then((response) => {
				if (response.ok) {
					setGameState('join');
				} else {
				}
			});
		}
	}

	function onKick() {
		if (player_to_kick.length == 0) {
			return;
		}
		if (confirm('Do you really what to kick ' + player_to_kick + '?') == true) {
			const response: Promise<Response> = deletePlayerFromGame(
				localStorage.getItem('game_name'),
				player_to_kick
			);
		}
	}

	function onEndGame() {
		if (confirm('Do you want to end the game for everybody?')) {
			const response: Promise<Response> = deleteGame(localStorage.getItem('game_name'));
			setGameState('join');
		}
	}
</script>

<main>
	{#if game_state == 'join'}
		<Join {setGameState} />
	{:else if game_state == 'answer'}
		<Answer
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{:else if game_state == 'answer_wait'}
		<AnswerWait
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{:else if game_state == 'results'}
		<Results
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
		/>
	{/if}
	<div style="padding: 200px;"></div>
	{#if game_state == 'answer'}
		<div>
			<Button text="Leave Game" onClick={onLeave} />
		</div>
		<div>
			<InputField bind:value={player_to_kick} text="player to kick" />
			<Button text="Kick" onClick={onKick} />
		</div>
	{/if}
	{#if game_state != 'join'}
		<div>
			<Button text="End Game" onClick={onEndGame} />
		</div>
	{/if}
</main>

<style>
	@import '../app.css';
</style>
