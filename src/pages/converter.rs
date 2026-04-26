use yew::prelude::*;
use crate::converters::{MeasureConverter, TemperatureConverter, SingleConversion};
use crate::models::MeasureUnit;
use wasm_bindgen::JsCast;

#[function_component(ConverterPage)]
pub fn converter_page() -> Html {
    let tab = use_state(|| "measure".to_string());
    let measure_amount = use_state(|| 100.0_f64);
    let measure_product = use_state(|| "wheat_flour".to_string());
    let measure_unit = use_state(|| MeasureUnit::Gram);
    let measure_result = use_state(|| Vec::<SingleConversion>::new());
    let temp_c = use_state(|| 180.0_f64);
    let temp_f = use_state(|| 356.0_f64);
    let pounds_amount = use_state(|| 1.0_f64);
    let pounds_result = use_state(|| 453.592_f64);
    let ounces_amount = use_state(|| 16.0_f64);
    let ounces_result = use_state(|| 453.592_f64);

    // === Конвертер мер ===
    let on_measure_convert = {
        let measure_amount = measure_amount.clone();
        let measure_product = measure_product.clone();
        let measure_unit = measure_unit.clone();
        let measure_result = measure_result.clone();
        Callback::from(move |_| {
            let conversions = MeasureConverter::convert(
                &measure_product,
                *measure_amount,
                &measure_unit,
            );
            measure_result.set(conversions.unwrap_or_default());
        })
    };

    let on_amount_change = {
        let measure_amount = measure_amount.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse() {
                measure_amount.set(val);
            }
        })
    };

    let on_product_change = {
        let measure_product = measure_product.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            measure_product.set(select.value());
        })
    };

    let on_unit_change = {
        let measure_unit = measure_unit.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let unit = match select.value().as_str() {
                "g" => MeasureUnit::Gram,
                "kg" => MeasureUnit::Kilogram,
                "oz" => MeasureUnit::Ounce,
                "lb" => MeasureUnit::Pound,
                "cup" => MeasureUnit::Cup,
                "tbsp" => MeasureUnit::Tablespoon,
                "tsp" => MeasureUnit::Teaspoon,
                "ml" => MeasureUnit::Ml,
                "l" => MeasureUnit::Liter,
                "stick" => MeasureUnit::StickOfButter,
                _ => MeasureUnit::Gram,
            };
            measure_unit.set(unit);
        })
    };

    // === Конвертер фунты/унции ===
    let on_pounds_change = {
        let pounds_amount = pounds_amount.clone();
        let pounds_result = pounds_result.clone();
        let ounces_amount = ounces_amount.clone();
        let ounces_result = ounces_result.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<f64>() {
                pounds_amount.set(val);
                let grams = val * 453.592;
                pounds_result.set(grams);
                ounces_amount.set(val * 16.0);
                ounces_result.set(grams);
            }
        })
    };

    let on_ounces_change = {
        let ounces_amount = ounces_amount.clone();
        let ounces_result = ounces_result.clone();
        let pounds_amount = pounds_amount.clone();
        let pounds_result = pounds_result.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<f64>() {
                ounces_amount.set(val);
                let grams = val * 28.3495;
                ounces_result.set(grams);
                pounds_amount.set(val / 16.0);
                pounds_result.set(grams);
            }
        })
    };

    // === Температура ===
    let on_c_change = {
        let temp_c = temp_c.clone();
        let temp_f = temp_f.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(c) = input.value().parse::<f64>() {
                temp_c.set(c);
                temp_f.set(TemperatureConverter::c_to_f(c));
            }
        })
    };

    let on_f_change = {
        let temp_c = temp_c.clone();
        let temp_f = temp_f.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(f) = input.value().parse::<f64>() {
                temp_f.set(f);
                temp_c.set(TemperatureConverter::f_to_c(f));
            }
        })
    };

    // === Переключатели табов ===
    let set_tab = |tab_val: &str| {
        let tab = tab.clone();
        let val = tab_val.to_string();
        Callback::from(move |_| tab.set(val.clone()))
    };

    html! {
        <div>
            <div class="card">
                <h2>{ "🔄 Конвертеры" }</h2>
                <div style="display: flex; gap: 8px; margin-bottom: 20px; flex-wrap: wrap;">
                    <button class={if *tab == "measure" { "btn btn-primary btn-sm" } else { "btn btn-outline btn-sm" }}
                        onclick={set_tab("measure")}>{ "⚖️ Меры" }</button>
                    <button class={if *tab == "pounds" { "btn btn-primary btn-sm" } else { "btn btn-outline btn-sm" }}
                        onclick={set_tab("pounds")}>{ "🇺🇸 Фунты/Унции" }</button>
                    <button class={if *tab == "temperature" { "btn btn-primary btn-sm" } else { "btn btn-outline btn-sm" }}
                        onclick={set_tab("temperature")}>{ "🌡️ Температура" }</button>
                    <button class={if *tab == "eggs" { "btn btn-primary btn-sm" } else { "btn btn-outline btn-sm" }}
                        onclick={set_tab("eggs")}>{ "🥚 Яйца" }</button>
                    <button class={if *tab == "sweetener" { "btn btn-primary btn-sm" } else { "btn btn-outline btn-sm" }}
                        onclick={set_tab("sweetener")}>{ "🍬 Сахарозаменители" }</button>
                    <button class={if *tab == "yeast" { "btn btn-primary btn-sm" } else { "btn btn-outline btn-sm" }}
                        onclick={set_tab("yeast")}>{ "🍞 Дрожжи" }</button>
                </div>

                // ========== КОНВЕРТЕР МЕР ==========
                if *tab == "measure" {
                    <div>
                        <h3>{ "⚖️ Конвертер мер и весов" }</h3>
                        <p style="color: var(--text-secondary); margin-bottom: 16px;">
                            { "Переводит любые единицы с учётом плотности продукта." }
                        </p>
                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                            <div class="form-group">
                                <label>{ "Продукт" }</label>
                                <select onchange={on_product_change}>
                                    <option value="wheat_flour">{ "Мука пшеничная" }</option>
                                    <option value="almond_flour">{ "Мука миндальная" }</option>
                                    <option value="sugar_white">{ "Сахар белый" }</option>
                                    <option value="powdered_sugar">{ "Сахарная пудра" }</option>
                                    <option value="butter">{ "Масло сливочное" }</option>
                                    <option value="vegetable_oil">{ "Масло растительное" }</option>
                                    <option value="milk_whole">{ "Молоко" }</option>
                                    <option value="cream_33">{ "Сливки 33%" }</option>
                                    <option value="honey">{ "Мёд" }</option>
                                    <option value="cocoa_powder">{ "Какао-порошок" }</option>
                                    <option value="water">{ "Вода" }</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label>{ "Количество" }</label>
                                <input type="number" value={(*measure_amount).to_string()} oninput={on_amount_change} min="0.1" step="any" />
                            </div>
                            <div class="form-group">
                                <label>{ "Единица" }</label>
                                <select onchange={on_unit_change}>
                                    <option value="g">{ "Граммы (г)" }</option>
                                    <option value="kg">{ "Килограммы (кг)" }</option>
                                    <option value="oz">{ "Унции (oz)" }</option>
                                    <option value="lb">{ "Фунты (lb)" }</option>
                                    <option value="cup">{ "Чашки (cup)" }</option>
                                    <option value="tbsp">{ "Столовые ложки" }</option>
                                    <option value="tsp">{ "Чайные ложки" }</option>
                                    <option value="ml">{ "Миллилитры (мл)" }</option>
                                    <option value="l">{ "Литры (л)" }</option>
                                    <option value="stick">{ "Стик масла (US)" }</option>
                                </select>
                            </div>
                        </div>
                        <button class="btn btn-primary" onclick={on_measure_convert} style="margin-top: 12px;">
                            { "🔄 Конвертировать" }
                        </button>

                        if !(*measure_result).is_empty() {
                            <div class="card" style="margin-top: 16px; background: rgba(0,210,160,0.05);">
                                <h4>{ "Результат:" }</h4>
                                <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(130px, 1fr)); gap: 8px; margin-top: 12px;">
                                    { for (*measure_result).iter().map(|c| {
                                        html! {
                                            <div style="padding: 12px; background: var(--bg); border-radius: 8px; text-align: center;">
                                                <strong style="font-size: 1.1rem;">{ format!("{:.1}", c.amount) }</strong>
                                                <div style="color: var(--text-secondary); font-size: 0.8rem;">{ &c.unit_label }</div>
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    </div>
                }

                // ========== ФУНТЫ / УНЦИИ ==========
                if *tab == "pounds" {
                    <div>
                        <h3>{ "🇺🇸 Фунты и унции в граммы" }</h3>
                        <p style="color: var(--text-secondary); margin-bottom: 16px;">
                            { "Связанный конвертер: меняйте фунты — унции и граммы пересчитываются автоматически." }
                        </p>

                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px;">
                            <div class="card" style="background: var(--bg);">
                                <div class="form-group">
                                    <label>{ "🇬🇧 Фунты (lb)" }</label>
                                    <input type="number" value={(*pounds_amount).to_string()} oninput={on_pounds_change} step="any" />
                                </div>
                                <div style="text-align: center; padding: 16px; background: rgba(108,92,231,0.1); border-radius: 8px;">
                                    <span style="color: var(--text-secondary);">{ "= " }</span>
                                    <strong style="font-size: 1.3rem; color: var(--accent);">{ format!("{:.1} г", *pounds_result) }</strong>
                                    <div style="color: var(--text-secondary); font-size: 0.8rem; margin-top: 4px;">{ format!("({:.2} кг)", *pounds_result / 1000.0) }</div>
                                </div>
                            </div>

                            <div class="card" style="background: var(--bg);">
                                <div class="form-group">
                                    <label>{ "🇺🇸 Унции (oz)" }</label>
                                    <input type="number" value={(*ounces_amount).to_string()} oninput={on_ounces_change} step="any" />
                                </div>
                                <div style="text-align: center; padding: 16px; background: rgba(0,206,201,0.1); border-radius: 8px;">
                                    <span style="color: var(--text-secondary);">{ "= " }</span>
                                    <strong style="font-size: 1.3rem; color: var(--accent);">{ format!("{:.1} г", *ounces_result) }</strong>
                                    <div style="color: var(--text-secondary); font-size: 0.8rem; margin-top: 4px;">{ format!("({:.2} фунта)", *ounces_amount / 16.0) }</div>
                                </div>
                            </div>
                        </div>

                        <div class="card" style="margin-top: 16px; background: rgba(253,203,110,0.1);">
                            <h4>{ "📋 Быстрые конверсии (US → Метрическая)" }</h4>
                            <table style="width: 100%; border-collapse: collapse; margin-top: 8px;">
                                <tr><td style="padding: 10px;">{ "1 фунт (lb)" }</td><td style="padding: 10px;"><strong>{ "= 453.6 г" }</strong></td><td style="padding: 10px; color: var(--text-secondary);">{ "= 16 унций" }</td></tr>
                                <tr><td style="padding: 10px;">{ "1 унция (oz)" }</td><td style="padding: 10px;"><strong>{ "= 28.35 г" }</strong></td><td style="padding: 10px; color: var(--text-secondary);">{ "= 1/16 фунта" }</td></tr>
                                <tr><td style="padding: 10px;">{ "1 стик масла" }</td><td style="padding: 10px;"><strong>{ "= 113.4 г" }</strong></td><td style="padding: 10px; color: var(--text-secondary);">{ "= 8 ст.л. = 4 oz" }</td></tr>
                                <tr><td style="padding: 10px;">{ "1 чашка муки" }</td><td style="padding: 10px;"><strong>{ "≈ 130 г" }</strong></td><td style="padding: 10px; color: var(--text-secondary);">{ "= 4.6 oz" }</td></tr>
                                <tr><td style="padding: 10px;">{ "1 чашка сахара" }</td><td style="padding: 10px;"><strong>{ "≈ 200 г" }</strong></td><td style="padding: 10px; color: var(--text-secondary);">{ "= 7.1 oz" }</td></tr>
                            </table>
                        </div>
                    </div>
                }

                // ========== ТЕМПЕРАТУРА ==========
                if *tab == "temperature" {
                    <div>
                        <h3>{ "🌡️ Конвертер температуры" }</h3>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                            <div class="form-group">
                                <label>{ "Цельсий (°C)" }</label>
                                <input type="number" value={(*temp_c).to_string()} oninput={on_c_change} />
                            </div>
                            <div class="form-group">
                                <label>{ "Фаренгейт (°F)" }</label>
                                <input type="number" value={(*temp_f).to_string()} oninput={on_f_change} />
                            </div>
                        </div>
                        <div class="card" style="background: rgba(253,203,110,0.1); margin-top: 16px;">
                            <h4>{ "📋 Режимы выпечки" }</h4>
                            <div style="display: grid; gap: 6px;">
                                <div style="padding: 8px;">{ "🌿 30°C / 85°F — Расстойка теста" }</div>
                                <div style="padding: 8px;">{ "🍪 165°C / 329°F — Бисквит женуаз" }</div>
                                <div style="padding: 8px;">{ "🍰 175°C / 347°F — Бисквиты, заварное" }</div>
                                <div style="padding: 8px;">{ "🔥 180°C / 356°F — Универсальная" }</div>
                                <div style="padding: 8px;">{ "🥐 200°C / 392°F — Слоёное тесто" }</div>
                                <div style="padding: 8px;">{ "🍞 220°C / 428°F — Хлеб с корочкой" }</div>
                                <div style="padding: 8px;">{ "🔥 250°C / 482°F — Пицца" }</div>
                            </div>
                        </div>
                    </div>
                }

                // ========== ЯЙЦА ==========
                if *tab == "eggs" {
                    <div class="card">
                        <h3>{ "🥚 Конвертер яиц" }</h3>
                        <table style="width: 100%; border-collapse: collapse;">
                            <tr><th>{ "Категория" }</th><th>{ "Вес" }</th><th>{ "3 яйца" }</th><th>{ "US" }</th></tr>
                            <tr><td>{ "СВ" }</td><td>{ "80 г" }</td><td>{ "240 г" }</td><td>{ "Jumbo" }</td></tr>
                            <tr><td>{ "С0" }</td><td>{ "70 г" }</td><td>{ "210 г" }</td><td>{ "XL" }</td></tr>
                            <tr><td>{ "С1" }</td><td>{ "60 г" }</td><td>{ "180 г" }</td><td>{ "Large" }</td></tr>
                            <tr><td>{ "С2" }</td><td>{ "50 г" }</td><td>{ "150 г" }</td><td>{ "Medium" }</td></tr>
                            <tr><td>{ "С3" }</td><td>{ "40 г" }</td><td>{ "120 г" }</td><td>{ "Small" }</td></tr>
                        </table>
                    </div>
                }

                // ========== САХАРОЗАМЕНИТЕЛИ ==========
                if *tab == "sweetener" {
                    <div class="card">
                        <h3>{ "🍬 Сахарозаменители" }</h3>
                        <table style="width: 100%; border-collapse: collapse;">
                            <tr><th>{ "Заменитель" }</th><th>{ "Сладость" }</th><th>{ "100 г сахара =" }</th><th>{ "Нюанс" }</th></tr>
                            <tr><td>{ "Эритритол" }</td><td>{ "0.7x" }</td><td>{ "143 г" }</td><td>{ "Холодок" }</td></tr>
                            <tr><td>{ "Стевия" }</td><td>{ "300x" }</td><td>{ "0.3 г" }</td><td>{ "Горчит" }</td></tr>
                            <tr><td>{ "Мёд" }</td><td>{ "1.3x" }</td><td>{ "77 г" }</td><td>{ "Жидкий" }</td></tr>
                            <tr><td>{ "Ксилит" }</td><td>{ "1.0x" }</td><td>{ "100 г" }</td><td>{ "1:1" }</td></tr>
                        </table>
                    </div>
                }

                // ========== ДРОЖЖИ ==========
                if *tab == "yeast" {
                    <div class="card">
                        <h3>{ "🍞 Дрожжи" }</h3>
                        <table style="width: 100%; border-collapse: collapse;">
                            <tr><th>{ "Тип" }</th><th>{ "10 г свежих =" }</th><th>{ "На 500 г муки" }</th></tr>
                            <tr><td>{ "Свежие" }</td><td>{ "10 г" }</td><td>{ "15-20 г" }</td></tr>
                            <tr><td>{ "Сухие активные" }</td><td>{ "4 г" }</td><td>{ "6-8 г" }</td></tr>
                            <tr><td>{ "Инстантные" }</td><td>{ "3.3 г" }</td><td>{ "5-7 г" }</td></tr>
                            <tr><td>{ "Осмотолерантные" }</td><td>{ "2.9 г" }</td><td>{ "4-6 г" }</td></tr>
                        </table>
                    </div>
                }
            </div>
        </div>
    }
}

