use yew::prelude::*;

#[function_component(PricingPage)]
pub fn pricing_page() -> Html {
    html! {
        <div>
            <div class="card" style="text-align: center;">
                <h2>{ "💎 Тарифы" }</h2>
                <p style="color: var(--gray);">
                    { "Выберите план, который подходит вашему кондитерскому бизнесу" }
                </p>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 20px;">

                // FREE
                <div class="pricing-card">
                    <h3>{ "🚀 Free" }</h3>
                    <div class="price" style="color: var(--gray);">{ "0 ₽" }</div>
                    <div class="period">{ "навсегда" }</div>
                    <ul>
                        <li>{ "✓ До 3 рецептов" }</li>
                        <li>{ "✓ Базовый калькулятор" }</li>
                        <li>{ "✓ Конвертер мер" }</li>
                        <li>{ "✓ Конвертер температур" }</li>
                        <li style="color: #ccc;">{ "✗ Конвертер форм" }</li>
                        <li style="color: #ccc;">{ "✗ Экспорт PDF" }</li>
                    </ul>
                    <button class="btn btn-outline" style="width: 100%;">{ "Начать бесплатно" }</button>
                </div>

                // PRO
                <div class="pricing-card" style="border-color: var(--primary); background: #fff8f8; transform: scale(1.03);">
                    <span class="badge" style="background: var(--primary); color: white; padding: 6px 14px; font-size: 0.85rem;">
                        { "⭐ Популярный" }
                    </span>
                    <h3 style="margin-top: 8px;">{ "👩‍🍳 Pro" }</h3>
                    <div class="price">{ "490 ₽" }</div>
                    <div class="period">{ "в месяц" }</div>
                    <ul>
                        <li>{ "✓ Неограниченно рецептов" }</li>
                        <li>{ "✓ ВСЕ конвертеры" }</li>
                        <li>{ "✓ Пересчёт форм" }</li>
                        <li>{ "✓ Конвертер яиц" }</li>
                        <li>{ "✓ Сахарозаменители" }</li>
                        <li>{ "✓ Экспорт в PDF" }</li>
                        <li>{ "✓ Без рекламы" }</li>
                    </ul>
                    <button class="btn btn-primary" style="width: 100%;">{ "Выбрать Pro" }</button>
                </div>

                // BUSINESS
                <div class="pricing-card">
                    <h3>{ "🏭 Business" }</h3>
                    <div class="price">{ "990 ₽" }</div>
                    <div class="period">{ "в месяц" }</div>
                    <ul>
                        <li>{ "✓ Всё из Pro" }</li>
                        <li>{ "✓ Аналитика рынка" }</li>
                        <li>{ "✓ Средние цены по городам" }</li>
                        <li>{ "✓ Статистика наценок" }</li>
                        <li>{ "✓ Приоритетная поддержка" }</li>
                        <li>{ "✓ API (скоро)" }</li>
                    </ul>
                    <button class="btn btn-outline" style="width: 100%;">{ "Выбрать Business" }</button>
                </div>
            </div>

            <div class="card" style="text-align: center; margin-top: 20px;">
                <h3>{ "💰 Годовая подписка — скидка 17%" }</h3>
                <p style="color: var(--gray);">
                    { "Pro: 4 900 ₽/год (вместо 5 880 ₽) • Business: 9 900 ₽/год (вместо 11 880 ₽)" }
                </p>
            </div>
        </div>
    }
}
