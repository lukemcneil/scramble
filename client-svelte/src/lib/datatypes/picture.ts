import { Player } from "./player";

export class Picture {
    prompt: string = "";
    url: string = "";

    constructor(name: string, url: string) {
        this.prompt = name;
        this.url = url;
    }
}