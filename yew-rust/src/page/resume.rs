use crate::components::Navigation;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Resume)]
pub fn resume_page() -> Html {
    html! {
        <body>
        <Navigation />
            <h1>{"Resume"}</h1>
            <p>{"This is the resume page. Add your resume content here."}</p>
            <Link<Route> to={Route::Home}>{"Back to Profile"}</Link<Route>>
        </body>
    }
}
