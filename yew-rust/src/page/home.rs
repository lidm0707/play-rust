use crate::components::Navigation;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {

        <body>
        <Navigation/>
            <main>
                <div class = "box-content-main">{"MY NAME IS MOO"} <br/> {"I am lerning in RUST"}</div>
                <div class="scroll-container">
                    <div class ="scroll-content">
                        <div class="scroll-item">{"Block 1"}</div>
                        <div class="scroll-item">{"Block 2"}</div>
                        <div class="scroll-item">{"Block 3"}</div>
                        <div class="scroll-item">{"Block 4"}</div>
                        <div class="scroll-item">{"Block 5"}</div>
                        <div class="scroll-item">{"Block 6"}</div>
                    </div>
                </div>
            </main>
        </body>


    }
}
