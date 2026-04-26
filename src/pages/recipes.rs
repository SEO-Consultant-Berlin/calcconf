use yew::prelude::*;
use crate::storage::AppStorage;
use crate::models::Recipe;

#[function_component(RecipeListPage)]
pub fn recipe_list_page() -> Html {
    let recipes = use_state(|| AppStorage::load_recipes());
    let show_delete = use_state(|| None::<String>);

    let on_delete_click = {
        let show_delete = show_delete.clone();
        Callback::from(move |id: String| {
            show_delete.set(Some(id));
        })
    };

    let on_confirm_delete = {
        let recipes = recipes.clone();
        let show_delete = show_delete.clone();
        Callback::from(move |_| {
            if let Some(ref id) = *show_delete {
                AppStorage::delete_recipe(id);
                recipes.set(AppStorage::load_recipes());
                show_delete.set(None);
            }
        })
    };

    let on_cancel_delete = {
        let show_delete = show_delete.clone();
        Callback::from(move |_| show_delete.set(None))
    };

    html! {
        <div>
            <div class="card" style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 12px;">
                <h2 style="margin: 0;">{ "📋 Мои рецепты" }</h2>
                <span style="color: var(--gray); font-size: 0.9rem;">
                    { format!("Сохранено: {}", recipes.len()) }
                </span>
            </div>

            if recipes.is_empty() {
                <div class="card" style="text-align: center; padding: 60px 24px;">
                    <div style="font-size: 4rem; margin-bottom: 16px;">{ "📝" }</div>
                    <h3>{ "У вас пока нет рецептов" }</h3>
                    <p style="color: var(--gray); margin-top: 8px;">
                        { "Перейдите в калькулятор, рассчитайте рецепт и сохраните его здесь." }
                    </p>
                </div>
            } else {
                <div class="card">
                    { for (*recipes).iter().map(|r| {
                        let on_click = {
                            let id = r.id.clone();
                            let on_delete_click = on_delete_click.clone();
                            Callback::from(move |_| on_delete_click.emit(id.clone()))
                        };
                        html! {
                            <div class="recipe-list-item">
                                <div>
                                    <strong style="font-size: 1.05rem;">{ &r.name }</strong>
                                    <div style="color: var(--gray); font-size: 0.85rem; margin-top: 2px;">
                                        { format!("{} порций • создан {}", r.portions, &r.created_at[..10]) }
                                    </div>
                                </div>
                                <button class="btn btn-danger btn-sm" onclick={on_click}>
                                    { "🗑️" }
                                </button>
                            </div>
                        }
                    })}
                </div>
            }

            // Модальное окно подтверждения удаления
            if let Some(ref id) = *show_delete {
                <div class="modal-overlay" onclick={on_cancel_delete.clone()}>
                    <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <h3>{ "Удалить рецепт?" }</h3>
                        <p style="color: var(--gray); margin: 16px 0;">
                            { "Это действие нельзя отменить. Рецепт будет удалён навсегда." }
                        </p>
                        <div style="display: flex; gap: 12px; justify-content: flex-end;">
                            <button class="btn btn-outline btn-sm" onclick={on_cancel_delete}>
                                { "Отмена" }
                            </button>
                            <button class="btn btn-danger btn-sm" onclick={on_confirm_delete}>
                                { "Удалить" }
                            </button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
