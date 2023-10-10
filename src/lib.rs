use yew::prelude::*;
use crate::router::{switch, Route};
use yew_router::prelude::*;

mod components;
mod router;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
     }
}
