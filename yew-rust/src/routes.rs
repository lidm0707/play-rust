use yew::prelude::*;
use yew_router::prelude::*;

use crate::page::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cal")]
    Calculate,
    #[at("/resume")]
    Resume,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Calculate => html! { <Calculate /> },
        Route::Resume => html! { <Resume /> },
    }
}
