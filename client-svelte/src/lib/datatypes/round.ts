import { Answer } from "./answer"

export class Round {
    letters: Array<string> = [];
    answers: Array<Answer> = [];

    constructor(letters: Array<string>) {
        this.letters = letters;
    }
}