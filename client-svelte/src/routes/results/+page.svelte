<script lang="ts">
	import Button from "$lib/Button.svelte";
	import type { Answer } from "$lib/datatypes/answer";
	import type { Guess } from "$lib/datatypes/guess";
	import { Score } from "$lib/datatypes/score";
	import { onMount } from "svelte";

    let name: any;
    let game_name: any;
    let base_server_path: any;
    let round_count: any;
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
            round_count = parseInt(round_count);
        }
        else {
            round_count = -1;
        }
    }

    function onNextRoundClick() {
        localStorage.setItem("round_count", round_count + 1 as string);
        window.location.href = localStorage.getItem("base_client_path") + "game";
    }

    async function getGameState() {
        const response: Response = await fetch(base_server_path + game_name, {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }

    async function getScore() {
        const response: Response = await fetch(base_server_path + game_name + "/score", {
            method: "GET",
            headers: {"Content-Type": "application/json"},
        })
        return response;
    }
    let scores: Array<Score> = [];

    function updateScore(score: Score) {
        scores = [...scores, score];
    }

    function getScores() {
        getScore().then((response) => response.json()).then((data) => {
            for (var prop in data) {
                updateScore(new Score(prop, data[prop]));
            }
        })
    }

    let answers: Array<Answer> = [];
    let correct_answer_map: Map<string, string> = new Map();
    let my_guess: Array<Answer> = [];
    let my_guess_map: Map<string, string> = new Map();
    async function getGame() {
        getGameState().then((response) => response.json()).then((data) => {

            answers = data.rounds[round_count].answers;
            answers.forEach((answer: Answer) => {
                correct_answer_map.set(answer.player, answer.answer);
            });

            data.rounds[round_count].guesses.forEach((guess: Guess) => {
                if (guess.player == name) {
                    my_guess = guess.answers;
                }
            });
            my_guess.forEach((answer: Answer) => {
                my_guess_map.set(answer.player, answer.answer);
            })

        })
    }

    onMount(() => {
        getGame();
        getScores();
    })
</script>

<style>
  @import '../../app.css';
</style>

<main>
    <h2>
        Results
    </h2>
    <div>
        {#each answers as answer}
            {#if answer.player != name}
                <div>
                    {answer.player}: {my_guess_map.get(answer.player)}
                    {#if answer.answer == my_guess_map.get(answer.player)}
                        ✅
                    {:else}
                        ❌ actually said: {answer.answer}
                    {/if}
                </div>
            {/if}
        {/each}
    </div>
    <div>
        {#each scores as score}
            <div>
                {score.player}: {score.score}
            </div>
        {/each}
    </div>
    <div>
        <Button text="Next Round" onClick={onNextRoundClick} />
    </div>
</main>