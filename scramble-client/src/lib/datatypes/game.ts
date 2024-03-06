import { GameSettings } from "./gameSettings";
import { Player } from "./player";
import { Round } from "./round";

export class Game {
    players: Array<Player> = [];
    rounds: Array<Round> = [];
    settings: GameSettings = new GameSettings();

    constructor(players: Array<Player>, rounds: Array<Round>) {
        this.players = players;
        this.rounds = rounds;
    }
}