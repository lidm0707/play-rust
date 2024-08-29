use crate::components::Navigation;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Calculate)]
pub fn calculate() -> Html {
    let counter1: UseStateHandle<i32> = use_state(|| 0);
    let counter2: UseStateHandle<i32> = use_state(|| 0);
    let header: UseStateHandle<&str> = use_state(|| "hello count");
    let onclick_add = {
        let counter1: UseStateHandle<i32> = counter1.clone();
        move |_| {
            let value: i32 = *counter1 + 1;
            counter1.set(value);
        }
    };

    let onclick_subtract = {
        let counter1: UseStateHandle<i32> = counter1.clone();
        let counter2: UseStateHandle<i32> = counter2.clone();
        move |_| {
            let value1: i32 = *counter1 + 1;
            let value2: i32 = *counter2 - 1;

            counter1.set(value1);
            counter2.set(value2);
        }
    };

    let onclick_head = {
        let header: UseStateHandle<&str> = header.clone();
        move |_| {
            header.set("change");
        }
    };

    html! {
        <body>
            <Navigation />
            <h1>{*header}</h1>
            <button onclick={onclick_head}>{ "head" }</button>
            <button onclick={onclick_add}>{ "+1" }</button>
            <button onclick={onclick_subtract}>{ "-1" }</button>
            <p>{ *counter1 }</p>
            <p>{ *counter2 }</p>
            <Link<Route> to={Route::Home}>{ "Go Home" }</Link<Route>>
        </body>
    }
}
