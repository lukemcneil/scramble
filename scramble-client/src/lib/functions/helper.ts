let name: any;
let game_name: any;
let base_server_path: any;
let base_client_path: any;
let round_count: any;
let has_answered: any = "false";

export function loadLocalStorage() {
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

        if (localStorage.getItem("base_client_path") != null) {
            base_client_path = localStorage.getItem("base_client_path");
        }
        else {
            base_client_path = "";
        }
        
        if (localStorage.getItem("round_count") != null) {
            round_count = localStorage.getItem("round_count");
        }
        else {
            round_count = "";
        }

        if (localStorage.getItem("has_answered") != null) {
            has_answered = localStorage.getItem("has_answered");
        }
        else {
            has_answered = "";
        }
    }
}

export function getName(): string{
    if (name === "undefined") {
        loadLocalStorage();
    }
    return name;
}

export function getGameName(): string{
    if (game_name === "undefined") {
        loadLocalStorage();
    }
    return game_name;
}

export function getBaseServerPath(): string{
    if (base_server_path === "undefined") {
        loadLocalStorage();
    }
    return base_server_path;
}

export function getBaseClientPath(): string{
    if (base_client_path === "undefined") {
        loadLocalStorage();
    }
    return base_client_path;
}

export function getRoundCount(): string{
    if (round_count === "undefined") {
        loadLocalStorage();
    }
    return round_count;
}

export function getHasAnswered(): string{
    if (has_answered === "undefined") {
        loadLocalStorage();
    }
    return has_answered;
}

export function setKey(key: string,  value: string) {
    localStorage.setItem(key, value);
    loadLocalStorage();
}

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}