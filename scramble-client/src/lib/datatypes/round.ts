import { Answer } from "./answer"

export class Round {
    letters: Array<string> = [];
    answers: Array<Answer> = [];
    guesses_used: Object = {};
    best_answers: Array<any> = [];

    constructor(letters: Array<string>) {
        this.letters = letters;
    }
}