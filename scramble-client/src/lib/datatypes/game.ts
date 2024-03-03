import { Player } from "./player";
import { Round } from "./round";

export class Game {
    players: Array<Player> = [];
    rounds: Array<Round> = [];

    constructor(players: Array<Player>, rounds: Array<Round>) {
        this.players = players;
        this.rounds = rounds;
    }

    // constructor(names: Array<string>, rounds: Array<object>) {
    //     this.players = new Array();
    //     names.forEach(name => {
    //        this.players.push(new Player(name)); 
    //     });
        
    //     this.rounds = new Array();
    //     rounds.forEach(round => {
    //         this.rounds.push(new Round(round.question))
    //     })
    // }
}