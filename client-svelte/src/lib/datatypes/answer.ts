import { Player } from "./player";

export class Answer {
    player: string = "";
    answer: string = "";

    constructor(name: string, answer: string) {
        this.player = name;
        this.answer = answer;
    }
}