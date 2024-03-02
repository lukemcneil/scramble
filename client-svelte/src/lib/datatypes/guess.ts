import { Player } from "./player";
import { Answer } from "./answer";

export class Guess {
    player: string = "";
    answers: Array<Answer> = [];

    constructor(name: string, answers: Array<Answer>) {
        this.player = name;
        this.answers = answers;
    }
}