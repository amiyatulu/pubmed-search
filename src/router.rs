use crate::components::search_page::SearchPage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    SearchPage,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::SearchPage => html! { <SearchPage />},
    }
}
