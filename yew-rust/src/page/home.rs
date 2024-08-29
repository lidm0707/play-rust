use crate::components::Navigation;
use yew::prelude::*;
 
#[function_component(Home)]
pub fn home() -> Html {
    html! {

        <body>
        <Navigation/>
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
