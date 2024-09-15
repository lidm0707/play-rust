use yew::prelude::*;
use crate::components::invoice_form::InvoiceForm;
mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"ERP Document Creator"}</h1>
            <InvoiceForm />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
