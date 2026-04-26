use yew::prelude::*;
use crate::i18n::Lang;

#[derive(Clone, PartialEq, Properties)]
pub struct LangProviderProps {
    pub children: Children,
}

#[function_component(LangProvider)]
pub fn lang_provider(props: &LangProviderProps) -> Html {
    let lang = use_state(|| Lang::Ru);

    let ctx = (lang.clone(), {
        let lang = lang.clone();
        Callback::from(move |new_lang: Lang| lang.set(new_lang))
    });

    html! {
        <ContextProvider<UseStateHandle<Lang>> context={lang.clone()}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<Lang>>>
    }
}
