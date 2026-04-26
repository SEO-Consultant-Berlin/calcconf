use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::i18n::{Lang, translate};

#[function_component(Nav)]
pub fn nav() -> Html {
    let menu_open = use_state(|| false);
    let lang = use_state(|| Lang::Ru);
    let t = |key: &str| translate(key, &lang);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(false))
    };

    html! {
        <>
            <header class="nav-header">
                <div class="nav-brand">
                    <Link<Route> to={Route::Home} classes="brand-link">
                        <span class="brand-icon">{ "🧁" }</span>
                        <span class="brand-text">{ "CalcConf" }</span>
                    </Link<Route>>
                </div>

                <div class="nav-right">
                    <button class="hamburger" onclick={toggle_menu}>
                        <span></span><span></span><span></span>
                    </button>
                </div>
            </header>

            if *menu_open {
                <div class="menu-overlay" onclick={close_menu.clone()}>
                    <div class="menu-dropdown" onclick={|e: web_sys::MouseEvent| e.stop_propagation()}>
                        <div class="menu-header">
                            <span class="brand-text-sm">{ "CalcConf" }</span>
                            <button class="menu-close" onclick={close_menu}>{ "✕" }</button>
                        </div>
                        <nav class="menu-links">
                            <Link<Route> to={Route::Home} classes="menu-link" onclick={close_menu.clone()}>
                                <span class="menu-emoji">{ "🧁" }</span>{ t("calculator") }
                            </Link<Route>>
                            <Link<Route> to={Route::Converter} classes="menu-link" onclick={close_menu.clone()}>
                                <span class="menu-emoji">{ "🔄" }</span>{ t("converters") }
                            </Link<Route>>
                            <Link<Route> to={Route::Recipes} classes="menu-link" onclick={close_menu.clone()}>
                                <span class="menu-emoji">{ "📋" }</span>{ t("recipes") }
                            </Link<Route>>
                            <Link<Route> to={Route::Pricing} classes="menu-link" onclick={close_menu.clone()}>
                                <span class="menu-emoji">{ "💎" }</span>{ t("pricing") }
                            </Link<Route>>
                            <Link<Route> to={Route::Admin} classes="menu-link" onclick={close_menu.clone()}>
                                <span class="menu-emoji">{ "📊" }</span>{ t("analytics") }
                            </Link<Route>>
                            <Link<Route> to={Route::Profile} classes="menu-link" onclick={close_menu.clone()}>
                                <span class="menu-emoji">{ "👤" }</span>{ t("profile") }
                            </Link<Route>>
                        </nav>
                        <div class="menu-lang">
                            <span>{ "Язык:" }</span>
                            <div class="lang-flags-menu">
                                { for vec![Lang::Ru, Lang::En, Lang::Es, Lang::De].iter().map(|l| {
                                    let lang_clone = lang.clone();
                                    let l_clone = l.clone();
                                    let onclick = Callback::from(move |_| lang_clone.set(l_clone.clone()));
                                    let active = *lang == *l;
                                    html! {
                                        <span class={if active { "flag-btn active" } else { "flag-btn" }}
                                            onclick={onclick}>{ l.as_str().to_uppercase() }</span>
                                    }
                                })}
                            </div>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
