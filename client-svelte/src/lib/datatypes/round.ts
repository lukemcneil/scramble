import { Player } from "./player"
import { Answer } from "./answer"
import { Guess } from "./guess"

export class Round {
    question: string | undefined;
    answers: Array<Answer> = [];
    guesses: Array<Guess> = [];

    constructor(question: string) {
        this.question = question;
    }
}