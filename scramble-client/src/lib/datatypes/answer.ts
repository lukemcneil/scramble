export class Answer {
    player: string = "";
    answer: string = "";
    score: number = 0;
    definition: string = "";

    constructor(name: string, answer: string) {
        this.player = name;
        this.answer = answer;
    }
}