use yew::prelude::*;
use crate::calculator::CostCalculator;
use crate::models::*;
use crate::storage::AppStorage;
use uuid::Uuid;
use web_sys::HtmlInputElement;

#[function_component(Home)]
pub fn home() -> Html {
    let recipe = use_state(|| CostCalculator::new_recipe("Новый рецепт".into()));
    let result = use_state(|| None::<CalculationResult>);
    let show_additional = use_state(|| false);
    let saved_message = use_state(|| String::new());
    let categories = use_state(|| RecipeCategory::all());

    let on_calculate = {
        let recipe = recipe.clone();
        let result = result.clone();
        Callback::from(move |_| { let calc = CostCalculator::calculate(&recipe); result.set(Some(calc)); })
    };

    let on_save = {
        let recipe = recipe.clone();
        let saved_message = saved_message.clone();
        Callback::from(move |_| { AppStorage::save_recipe(&recipe); saved_message.set("✅ Рецепт сохранён!".into()); })
    };

    let on_name_change = {
        let recipe = recipe.clone();
        Callback::from(move |e: InputEvent| { let input: HtmlInputElement = e.target_unchecked_into(); let mut r = (*recipe).clone(); r.name = input.value(); recipe.set(r); })
    };

    let on_category_change = {
        let recipe = recipe.clone();
        Callback::from(move |e: Event| { let select: web_sys::HtmlSelectElement = e.target_unchecked_into(); let mut r = (*recipe).clone(); let val = select.value(); let cats = RecipeCategory::all(); r.category = cats.into_iter().find(|c| c.label() == val); recipe.set(r); })
    };

    let on_portions_change = {
        let recipe = recipe.clone();
        Callback::from(move |e: InputEvent| { let input: HtmlInputElement = e.target_unchecked_into(); if let Ok(val) = input.value().parse::<f64>() { let mut r = (*recipe).clone(); r.portions = val; recipe.set(r); } })
    };

    let on_weight_change = {
        let recipe = recipe.clone();
        Callback::from(move |e: InputEvent| { let input: HtmlInputElement = e.target_unchecked_into(); if let Ok(val) = input.value().parse::<f64>() { let mut r = (*recipe).clone(); r.total_weight_kg = val; recipe.set(r); } })
    };

    let on_add_ingredient = {
        let recipe = recipe.clone();
        Callback::from(move |_| { let mut r = (*recipe).clone(); r.ingredients.push(Ingredient { id: Uuid::new_v4().to_string(), name: "Новый ингредиент".into(), price_per_unit: 100.0, amount: 100.0, unit: MeasureUnit::Gram }); recipe.set(r); })
    };

    let on_toggle_additional = {
        let show_additional = show_additional.clone();
        Callback::from(move |_| show_additional.set(!*show_additional))
    };

    let ingredient_style = "padding: 8px 10px; background: var(--bg-card); border: 1px solid var(--border); border-radius: 6px; color: var(--text); font-size: 0.9rem; width: 100%;";

    html! {
        <div>
            <div class="card">
                <h2>{ "🧁 Калькулятор себестоимости" }</h2>
                <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 12px;">
                    <div class="form-group"><label>{ "Название рецепта" }</label><input type="text" value={(*recipe).name.clone()} oninput={on_name_change} /></div>
                    <div class="form-group"><label>{ "Категория" }</label><select onchange={on_category_change}><option value="">{ "Без категории" }</option>{ for (*categories).iter().map(|c| html! { <option value={c.label().to_string()}>{ c.label() }</option> })}</select></div>
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                    <div class="form-group"><label>{ "Порций" }</label><input type="number" value={(*recipe).portions.to_string()} oninput={on_portions_change} min="1" step="1" /></div>
                    <div class="form-group"><label>{ "Вес (кг)" }</label><input type="number" value={(*recipe).total_weight_kg.to_string()} oninput={on_weight_change} min="0.1" step="0.1" /></div>
                </div>
                <div style="margin: 20px 0;">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                        <h3 style="margin: 0; color: var(--text);">{ "📝 Ингредиенты" }</h3>
                        <button class="btn btn-secondary btn-sm" onclick={on_add_ingredient}>{ "+ Добавить" }</button>
                    </div>
                    { for (*recipe).ingredients.iter().enumerate().map(|(idx, ing)| {
                        let r1 = recipe.clone(); let r2 = recipe.clone(); let r3 = recipe.clone(); let r4 = recipe.clone(); let r5 = recipe.clone();
                        html! {
                            <div key={ing.id.clone()} style="display: grid; grid-template-columns: 2fr 1fr 1fr 0.8fr 40px; gap: 6px; padding: 8px; background: var(--bg); border-radius: 6px; align-items: center;">
                                <input type="text" value={ing.name.clone()} oninput={Callback::from(move |e: InputEvent| { let input: HtmlInputElement = e.target_unchecked_into(); let mut r = (*r1).clone(); r.ingredients[idx].name = input.value(); r1.set(r); })} style={ingredient_style} />
                                <input type="number" value={ing.amount.to_string()} oninput={Callback::from(move |e: InputEvent| { let input: HtmlInputElement = e.target_unchecked_into(); if let Ok(val) = input.value().parse::<f64>() { let mut r = (*r2).clone(); r.ingredients[idx].amount = val; r2.set(r); } })} step="any" style={ingredient_style} />
                                <input type="number" value={ing.price_per_unit.to_string()} oninput={Callback::from(move |e: InputEvent| { let input: HtmlInputElement = e.target_unchecked_into(); if let Ok(val) = input.value().parse::<f64>() { let mut r = (*r3).clone(); r.ingredients[idx].price_per_unit = val; r3.set(r); } })} step="any" style={ingredient_style} />
                                <select onchange={Callback::from(move |e: Event| { let select: web_sys::HtmlSelectElement = e.target_unchecked_into(); let mut r = (*r4).clone(); r.ingredients[idx].unit = match select.value().as_str() { "kg" => MeasureUnit::Kilogram, "g" => MeasureUnit::Gram, "ml" => MeasureUnit::Ml, "l" => MeasureUnit::Liter, "pc" => MeasureUnit::Piece, "tbsp" => MeasureUnit::Tablespoon, "tsp" => MeasureUnit::Teaspoon, _ => MeasureUnit::Gram }; r4.set(r); })} style="padding: 8px 6px; background: var(--bg-card); border: 1px solid var(--border); border-radius: 6px; color: var(--text); font-size: 0.85rem; width: 100%;">
                                    <option value="kg">{"кг"}</option><option value="g">{"г"}</option><option value="ml">{"мл"}</option><option value="l">{"л"}</option><option value="pc">{"шт"}</option>
                                </select>
                                <button onclick={Callback::from(move |_| { let mut r = (*r5).clone(); r.ingredients.remove(idx); r5.set(r); })} style="background: none; border: none; cursor: pointer; font-size: 1.2rem; color: var(--danger); padding: 4px;">{"✕"}</button>
                            </div>
                        }
                    })}
                </div>
                <button class="btn btn-outline btn-sm" onclick={on_toggle_additional}>{ if *show_additional { "▲ Скрыть" } else { "▼ Доп. расходы" } }</button>
                <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                    <button class="btn btn-primary" onclick={on_calculate}>{ "📊 Рассчитать" }</button>
                    <button class="btn btn-secondary" onclick={on_save}>{ "💾 Сохранить" }</button>
                </div>
            </div>
            if let Some(ref res) = *result {
                <div class="card">
                    <h2>{ "💰 Результаты" }</h2>
                    <div class="result-grid">
                        <div class="result-item"><div class="value">{ format!("{:.2} ₽", res.ingredients_cost) }</div><div class="label">{"Ингредиенты"}</div></div>
                        <div class="result-item"><div class="value">{ format!("{:.2} ₽", res.total_cost) }</div><div class="label">{"Себестоимость"}</div></div>
                        <div class="result-item"><div class="value">{ format!("{:.2} ₽", res.cost_per_portion) }</div><div class="label">{"За порцию"}</div></div>
                        <div class="result-item"><div class="value">{ format!("{:.2} ₽", res.recommended_price) }</div><div class="label">{"Цена продажи"}</div></div>
                        <div class="result-item"><div class="value">{ format!("{:.2} ₽", res.margin_per_portion) }</div><div class="label">{"Маржа"}</div></div>
                        <div class="result-item total-cost"><div class="value">{ format!("{:.2} ₽", res.net_profit) }</div><div class="label">{"Чистая прибыль"}</div></div>
                    </div>
                </div>
            }
        </div>
    }
}
