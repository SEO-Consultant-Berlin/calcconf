use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::city_data::{all_cities, CityPrices};

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    let all = use_state(|| all_cities());
    let selected = use_state(|| "Москва".to_string());
    let search_query = use_state(|| String::new());

    let filtered: Vec<String> = (*all).iter()
        .filter(|c| {
            let q = (*search_query).to_lowercase();
            q.is_empty() || c.name.to_lowercase().contains(&q) || c.country.to_lowercase().contains(&q)
        })
        .map(|c| c.name.clone())
        .collect();

    let cur_name = (*selected).clone();
    let cur = (*all).iter().find(|c| c.name == cur_name).unwrap_or(&(*all)[0]);

    let on_search = {
        let sq = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            sq.set(input.value());
        })
    };

    let ing_names = ["Мука 1 кг", "Сахар 1 кг", "Масло 82% 1 кг", "Яйца 10 шт", "Сливки 33% 1 л", "Творожный сыр 1 кг", "Шоколад 1 кг", "Миндальная мука 1 кг"];
    let cur_vals: [f64; 8] = [cur.flour, cur.sugar, cur.butter, cur.eggs, cur.cream, cur.cheese, cur.chocolate, cur.almond_flour];

    html! {
        <div>
            <div class="card">
                <h2>{ "📊 Аналитика рынка" }</h2>
                <p style="color: var(--text-secondary);">
                    { format!("Город: {} • Страна: {} • Валюта: {}", cur.name, cur.country, cur.currency) }
                </p>
                <div class="form-group" style="margin-top: 16px;">
                    <label>{ "🔍 Поиск города" }</label>
                    <input type="text" placeholder="Введите название..." value={(*search_query).clone()} oninput={on_search} />
                </div>
                if !(*search_query).is_empty() {
                    <div style="max-height: 300px; overflow-y: auto; margin-top: 8px; background: var(--bg); border-radius: var(--radius-sm); border: 1px solid var(--border);">
                        { for filtered.iter().map(|name| {
                            let c = (*all).iter().find(|x| &x.name == name).unwrap();
                            let sel = selected.clone();
                            let name_clone = name.clone();
                            let bg = if *name == cur_name { "background: rgba(108,92,231,0.15);" } else { "" };
                            html! {
                                <div style={format!("padding: 12px 16px; cursor: pointer; border-bottom: 1px solid var(--border); {}", bg)}
                                    onclick={Callback::from(move |_| sel.set(name_clone.clone()))}>
                                    <strong style="color: var(--text);">{ &c.name }</strong>
                                    <span style="color: var(--text-secondary); margin-left: 8px;">{ format!("{} ({})", c.country, c.currency) }</span>
                                </div>
                            }
                        })}
                    </div>
                }
            </div>

            <div class="card">
                <h3>{ format!("🏙️ Цены — {} ({})", cur.name, cur.currency) }</h3>
                <p style="color: var(--text-secondary); margin-bottom: 16px;">
                    { format!("Курс: 1 {} = {:.2} ₽", cur.currency, cur.rate_to_rub) }
                </p>
                <table>
                    <tr><th>{ "Ингредиент" }</th><th>{ "Местная цена" }</th><th>{ "В рублях" }</th></tr>
                    { for (0..8).map(|i| {
                        html! {
                            <tr>
                                <td><strong>{ ing_names[i] }</strong></td>
                                <td style="color: var(--accent); font-weight: 700; font-size: 1.1rem;">{ cur.format_price(cur_vals[i]) }</td>
                                <td style="color: var(--text-secondary);">{ format!("≈ {:.0} ₽", cur.in_rub(cur_vals[i])) }</td>
                            </tr>
                        }
                    })}
                </table>
            </div>
        </div>
    }
}
