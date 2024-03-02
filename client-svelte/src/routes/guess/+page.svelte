<script lang="ts">
	import Button from "$lib/Button.svelte";
	import type { Game } from "$lib/datatypes/game";
	import type { Round } from "$lib/datatypes/round";
	import { Answer } from "$lib/datatypes/answer";
	import { onMount } from "svelte";
	import { Guess } from "$lib/datatypes/guess";
	import Dropdown from "$lib/Dropdown.svelte";
    
    let name: any;
    let game_name: any;
    let base_server_path: any;
    let round_count: any;
    let has_guessed: any = "false";
    if (typeof localStorage !== "undefined") {
        if (localStorage.getItem("name") != null) {
            name = localStorage.getItem("name");
        }
        else {
            name = "";
        }
    
        if (localStorage.getItem("game_name") != null) {
            game_name = localStorage.getItem("game_name");
        }
        else {
            game_name = "";
        }

        if (localStorage.getItem("base_server_path") != null) {
            base_server_path = localStorage.getItem("base_server_path");
        }
        else {
            base_server_path = "";
        }

        if (localStorage.getItem("round_count") != null) {
            round_count = localStorage.getItem("round_count");
        }
        else {
            round_count = "";
        }

        if (localStorage.getItem("has_guessed") != null) {
            has_guessed = localStorage.getItem("has_guessed");
        }
        else {
            has_guessed = "";
        }
    }

    async function getGameState() {
        const response: Response = await fetch(base_server_path + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }

    let get_game_interval_ms: number = 1000;
    function sleep(ms: number) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async function getGameLoop() {
        getGame();
        await sleep(get_game_interval_ms);
        getGameLoop();
    }
    
    onMount(() => {
        getGameLoop();
    })
    
    let game: Game;
    let players: Array<string> = [];
    let other_players: Array<string> = [];
    let answers: Array<Answer> = [];
    let rounds: Array<Round> = [];
    let guess_player_list: Array<string> = [];
    let guess: Guess = new Guess(name, []);
    let has_everybody_guessed: boolean = false;

    async function getGame() {
        getGameState().then((response) => response.json()).then((data) => {
            game = data as Game;
            console.log(game);
            players = data.players;
            players.forEach((player: string) => {
                if (player != name){
                    other_players.push(player);
                }
            });
            if (answers.length == 0) {
                answers = data.rounds[data.rounds.length - 1].answers;
            }
            console.log(data.rounds[round_count].guesses.length);
            has_everybody_guessed = data.rounds[round_count].guesses.length == players.length;
            if (has_everybody_guessed) {
                localStorage.setItem("has_guessed", "false");
                window.location.href = localStorage.getItem("base_client_path") + "results";
            }
            rounds = game.rounds;
        })
    }

    async function postGuess() {
        console.log(guess);
        const response: Response = await fetch(base_server_path + game_name + "/guess", {
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify(guess),
        })
        return response;
    }

    function onSubmit() {
        console.log(guess_player_list);
        answers.forEach((answer, i) => {
            if (answer.player != name) {
                guess.answers.push(new Answer(guess_player_list[i], answer.answer));    
            }
        });
        const response: Promise<Response> = postGuess();
        response.then((response) => {
            if (response.ok) {
                has_guessed = "true";
                localStorage.setItem("has_guessed", "true");
            }
            else {
                console.log("not ok");
            }
        })
    }

    function onClickPlayer(name: string) {
        guess.answers.push(new Answer(name, answers[0].answer));
        answers.shift();
    }
</script>

<style>
  @import '../../app.css';
</style>

<main>
    <h2>
       Guess who said what
    </h2>
    {#if has_guessed != "true"}
        <div>
            {#each answers as answer, i}
                {#if answer.player != name}
                    <div>
                        {answer.answer}
                        <Dropdown bind:selected={guess_player_list[i]} options={other_players} />
                    </div>
                {/if}
            {/each}
        </div>
        <div>
            <Button text="Submit" onClick={onSubmit} />
        </div>
    {/if}
    {#if has_guessed == "true"}
        <div>
            wait for the other people
        </div>
    {/if}
</main>