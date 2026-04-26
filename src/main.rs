mod models;
mod calculator;
mod converters;
mod storage;
mod i18n;
mod city_data;
mod pages;
mod components;
use yew::prelude::*;
use yew_router::prelude::*;
use pages::{Home, ConverterPage, RecipeListPage, PricingPage, AdminPage, ProfilePage};
use components::Nav;
use wasm_bindgen::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")] Home,
    #[at("/converter")] Converter,
    #[at("/recipes")] Recipes,
    #[at("/pricing")] Pricing,
    #[at("/admin")] Admin,
    #[at("/profile")] Profile,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Converter => html! { <ConverterPage /> },
        Route::Recipes => html! { <RecipeListPage /> },
        Route::Pricing => html! { <PricingPage /> },
        Route::Admin => html! { <AdminPage /> },
        Route::Profile => html! { <ProfilePage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
