<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { putCreateGame, postJoinGame } from '$lib/functions/requests';

	export let setGameState: (new_state: string) => void;
	let name: string;
	let game_name: string;

	let error_message: string = '';
	let no_name_error_message = 'no name';
	let no_game_room_error_message = 'no game room name';
	let game_already_exists_error_message = 'this game already exists';
	let number_of_tiles: number = 7;
	let number_of_lookups: number = 2;

	async function onClickCreateGame() {
		if (name == '') {
			error_message = no_name_error_message;
			return;
		}
		if (game_name == '') {
			error_message = no_game_room_error_message;
			return;
		}
		const response: Promise<Response> = putCreateGame(
			game_name,
			name,
			number_of_tiles,
			number_of_lookups
		);
		response.then((response) => {
			if (response.ok) {
				localStorage.setItem('name', name);
				localStorage.setItem('game_name', game_name);
				setGameState('answer');
			} else {
				if (response.status == 409) {
					error_message = game_already_exists_error_message;
				}
				error_message = 'some other error when making a game';
			}
		});
	}

	async function onClickJoinGame() {
		if (name == '') {
			error_message = no_name_error_message;
			return;
		}
		if (game_name == '') {
			error_message = no_game_room_error_message;
			return;
		}
		const response: Promise<Response> = postJoinGame(game_name, name);
		response.then((response) => {
			if (response.ok) {
				localStorage.setItem('name', name);
				localStorage.setItem('game_name', game_name);
				setGameState('answer');
			}
		});
	}
</script>

<main>
	<h1>Scramble</h1>
	<div>
		<InputField bind:value={name} text="enter your name" />
	</div>

	<div>
		<InputField bind:value={game_name} text="enter the game room" />
	</div>

	<div>
		<Button text="Join Game" onClick={onClickJoinGame} />
	</div>

	<div>
		<Button text="Create Game" onClick={onClickCreateGame} />
	</div>

	<h3>Create Game Settings</h3>
	<div>
		Tiles: <input
			type="number"
			bind:value={number_of_tiles}
			min="2"
			max="20"
			style="width: 50px;"
		/>
	</div>
	<div>
		Lookups: <input
			type="number"
			bind:value={number_of_lookups}
			min="1"
			max="10"
			style="width: 50px;"
		/>
	</div>

	<div>
		<div>
			{error_message}
		</div>
	</div>
</main>

<style>
	@import '../../app.css';
</style>
