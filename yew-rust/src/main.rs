mod routes;
mod components;
mod page;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
