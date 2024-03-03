<script lang="ts">
	import { deletePlayerFromGame } from '$lib/functions/requests';
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { text } from '@sveltejs/kit';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	let player_to_kick: string;

	function onLeave() {
		if (confirm('Do you really want to leave the game?') == true) {
			const response: Promise<Response> = deletePlayerFromGame(game_name, name);
			response.then((response) => {
				if (response.ok) {
					setGameState('join');
				} else {
					setGameState('join');
				}
			});
		}
	}

	function onKick() {
		if (player_to_kick.length == 0) {
			return;
		}
		if (confirm('Do you really what to kick ' + player_to_kick + '?') == true) {
			const response: Promise<Response> = deletePlayerFromGame(game_name, player_to_kick);
		}
	}

	function reset() {
		setGameState('join');
	}
</script>

<main>
	<div style="padding-top: 10em;">
		<Button text="Leave Game" onClick={onLeave} />
	</div>
	<div>
		<InputField bind:value={player_to_kick} text="player to kick" />
		<Button text="Kick" onClick={onKick} />
	</div>
	<!-- <div>
		<Button text="Reset" onClick={reset} />
	</div> -->
</main>
