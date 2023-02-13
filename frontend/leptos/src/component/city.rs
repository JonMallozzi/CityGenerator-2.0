use leptos::*;
use leptos_meta::*;
use leptos::create_effect;

use game::game::run;

#[component]
pub fn City(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    
    create_effect(cx, move |_| {
        run();
    });

    view! {
        cx,
        <div>
            <h2> "Game" </h2>
            <div id="wasm"></div>
            <a href="/"> "back" </a>
        </div>
    }
}

