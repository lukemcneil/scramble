#![allow(non_snake_case)]
use log::{info, LevelFilter};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let interval_started = use_state(&cx, || false);
    let state = use_state(&cx, || 0);

    if !*interval_started.get() {
        interval_started.set(true);
        let state = state.clone();
        gloo_timers::callback::Interval::new(1_000, move || {
            let _ = get_game();
            info!("test");
            state.with_mut(|s| *s += 2);
        })
        .forget();
    }

    cx.render(rsx! {
        div {
            "{state}"
        }
    })
}

async fn get_game() -> Result<(), reqwest::Error> {
    let body = reqwest::get("localhost:8000/game/test")
        .await?
        .text()
        .await?;
    info!("body = {:?}", body);
    Ok(())
}
