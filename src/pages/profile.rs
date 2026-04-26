use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let bakery_name = use_state(|| String::from("Моя кондитерская"));
    let owner_name = use_state(|| String::from("Анна"));
    let email = use_state(|| String::from("anna@example.com"));
    let phone = use_state(|| String::from("+7 (999) 123-45-67"));
    let city = use_state(|| String::from("Москва"));
    let subscription = use_state(|| String::from("Free"));
    let recipes_count = use_state(|| 0);
    let edit_mode = use_state(|| false);
    let saved = use_state(|| false);

    let on_toggle_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| edit_mode.set(!*edit_mode))
    };

    let on_save = {
        let saved = saved.clone();
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            saved.set(true);
            edit_mode.set(false);
        })
    };

    let on_upgrade = Callback::from(|_| {
        let window = web_sys::window().unwrap();
        window.location().set_href("/pricing").unwrap();
    });

    let make_input = |label: &str, value: String, state: UseStateHandle<String>, edit: bool| {
        let state_clone = state.clone();
        let on_change = Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state_clone.set(input.value());
        });

        html! {
            <div class="form-group">
                <label>{ label }</label>
                if edit {
                    <input type="text" value={value} oninput={on_change} />
                } else {
                    <div style="padding: 12px 16px; background: var(--bg); border-radius: var(--radius-sm); border: 1px solid var(--border);">
                        { value }
                    </div>
                }
            </div>
        }
    };

    html! {
        <div>
            // ========== ШАПКА ПРОФИЛЯ ==========
            <div class="card" style="text-align: center;">
                <div style="width: 80px; height: 80px; border-radius: 50%; background: var(--gradient-1); margin: 0 auto 16px; display: flex; align-items: center; justify-content: center; font-size: 2rem;">
                    { "👩‍🍳" }
                </div>
                <h2>{ (*bakery_name).clone() }</h2>
                <p style="color: var(--text-secondary);">{ (*owner_name).clone() }</p>
                <div style="display: flex; gap: 8px; justify-content: center; margin-top: 12px; flex-wrap: wrap;">
                    <span class="badge" style="background: var(--gradient-1); color: white; padding: 6px 16px;">
                        { format!("⭐ {}", (*subscription).clone()) }
                    </span>
                    <span class="badge" style="background: var(--bg-card); color: var(--accent); border: 1px solid var(--accent); padding: 6px 16px;">
                        { format!("📋 {} рецептов", *recipes_count) }
                    </span>
                </div>
            </div>

            // ========== ДАННЫЕ ПРОФИЛЯ ==========
            <div class="card">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                    <h3 style="margin: 0;">{ "👤 Личные данные" }</h3>
                    <button class="btn btn-outline btn-sm" onclick={on_toggle_edit}>
                        { if *edit_mode { "✕ Отмена" } else { "✏️ Редактировать" } }
                    </button>
                </div>

                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                    { make_input("Название кондитерской", (*bakery_name).clone(), bakery_name.clone(), *edit_mode) }
                    { make_input("Ваше имя", (*owner_name).clone(), owner_name.clone(), *edit_mode) }
                    { make_input("Email", (*email).clone(), email.clone(), *edit_mode) }
                    { make_input("Телефон", (*phone).clone(), phone.clone(), *edit_mode) }
                    { make_input("Город", (*city).clone(), city.clone(), *edit_mode) }
                </div>

                if *edit_mode {
                    <button class="btn btn-primary" style="margin-top: 16px;" onclick={on_save}>
                        { "💾 Сохранить" }
                    </button>
                }

                if *saved {
                    <div style="margin-top: 12px; padding: 10px; background: rgba(0,210,160,0.1); border-radius: 8px; color: var(--success);">
                        { "✅ Данные сохранены!" }
                    </div>
                }
            </div>

            // ========== ПОДПИСКА ==========
            <div class="card">
                <h3>{ "💎 Ваша подписка" }</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px; margin-top: 16px;">
                    <div style="padding: 20px; background: var(--bg); border-radius: var(--radius-sm); border: 1px solid var(--border); text-align: center;">
                        <div style="font-size: 2rem; margin-bottom: 8px;">{ "🚀" }</div>
                        <h4>{ "Free" }</h4>
                        <p style="color: var(--text-secondary); font-size: 0.85rem; margin: 8px 0;">{ "Базовый доступ" }</p>
                        <p style="font-size: 0.8rem; color: var(--text-secondary);">{ "3 рецепта, базовые конвертеры" }</p>
                    </div>
                    <div style="padding: 20px; background: rgba(108,92,231,0.1); border-radius: var(--radius-sm); border: 2px solid var(--primary); text-align: center;">
                        <div style="font-size: 2rem; margin-bottom: 8px;">{ "⭐" }</div>
                        <h4 style="color: var(--primary-light);">{ "Pro" }</h4>
                        <p style="font-size: 1.5rem; font-weight: 700; color: var(--accent); margin: 8px 0;">{ "490 ₽/мес" }</p>
                        <p style="font-size: 0.8rem; color: var(--text-secondary);">{ "Всё включено, без ограничений" }</p>
                    </div>
                    <div style="padding: 20px; background: var(--bg); border-radius: var(--radius-sm); border: 1px solid var(--border); text-align: center;">
                        <div style="font-size: 2rem; margin-bottom: 8px;">{ "🏭" }</div>
                        <h4>{ "Business" }</h4>
                        <p style="font-size: 1.5rem; font-weight: 700; color: var(--accent); margin: 8px 0;">{ "990 ₽/мес" }</p>
                        <p style="font-size: 0.8rem; color: var(--text-secondary);">{ "Аналитика, API, поддержка" }</p>
                    </div>
                </div>
                <button class="btn btn-primary" style="margin-top: 16px; width: 100%;" onclick={on_upgrade}>
                    { "⬆️ Улучшить тариф" }
                </button>
            </div>

            // ========== СТАТИСТИКА ==========
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 16px;">
                <div class="card" style="text-align: center;">
                    <div style="font-size: 2.5rem;">{ "📋" }</div>
                    <div style="font-size: 2rem; font-weight: 700; background: var(--gradient-1); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">{ "0" }</div>
                    <div style="color: var(--text-secondary);">{ "Рецептов" }</div>
                </div>
                <div class="card" style="text-align: center;">
                    <div style="font-size: 2.5rem;">{ "💰" }</div>
                    <div style="font-size: 2rem; font-weight: 700; background: var(--gradient-3); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">{ "0 ₽" }</div>
                    <div style="color: var(--text-secondary);">{ "Прибыль" }</div>
                </div>
                <div class="card" style="text-align: center;">
                    <div style="font-size: 2.5rem;">{ "📅" }</div>
                    <div style="font-size: 1.2rem; font-weight: 700; color: var(--text); margin-top: 8px;">{ "С апреля 2026" }</div>
                    <div style="color: var(--text-secondary);">{ "С нами" }</div>
                </div>
            </div>

            // ========== ВЫХОД ==========
            <div class="card" style="text-align: center;">
                <p style="color: var(--text-secondary); margin-bottom: 12px;">
                    { "Хотите выйти из аккаунта? Все данные хранятся локально в вашем браузере." }
                </p>
                <button class="btn btn-danger btn-sm">
                    { "🚪 Выйти" }
                </button>
            </div>
        </div>
    }
}
