use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let navigator = use_navigator().unwrap();

    let create_onclick = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&route))
    };

    html! {
        <nav class="container-nav">
            <div class="nav-item">
                <span onclick={create_onclick(Route::Home)}>{ "HOME" }</span>
            </div>
            <div class="nav-item">
                <span onclick={create_onclick(Route::Resume)}>{ "RESUME" }</span>
            </div>
            <div class="nav-item">
                <span onclick={create_onclick(Route::Calculate)}>{ "CALCULATOR" }</span>
            </div>
        </nav>
    }
}