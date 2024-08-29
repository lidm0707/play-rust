use crate::components::Navigation;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {

        <body>
        <div class = "container-nav"><Navigation/></div>
            <main>
                <div class = "box-content">{"HELLO RUST"}</div>
                <div class = "box-content">{"HELLO RUST"}</div>
                <div class = "box-content">{"HELLO RUST"}</div>
                <div class = "box-content">{"HELLO RUST"}</div>
                <div class = "box-content">{"HELLO RUST"}</div>
                <div class = "box-content">{"HELLO RUST"}</div>
            </main>
        </body>


    }
}
