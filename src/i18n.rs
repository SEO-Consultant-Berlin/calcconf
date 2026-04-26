use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Lang { Ru, En, Es, De }

impl Lang {
    pub fn as_str(&self) -> &str {
        match self { Lang::Ru => "ru", Lang::En => "en", Lang::Es => "es", Lang::De => "de" }
    }
}

pub fn translate(key: &str, lang: &Lang) -> String {
    let dict: HashMap<&str, (&str, &str, &str, &str)> = HashMap::from([
        // Навигация
        ("calculator", ("Калькулятор", "Calculator", "Calculadora", "Rechner")),
        ("converters", ("Конвертеры", "Converters", "Convertidores", "Konverter")),
        ("recipes", ("Рецепты", "Recipes", "Recetas", "Rezepte")),
        ("pricing", ("Тарифы", "Pricing", "Precios", "Preise")),
        ("analytics", ("Аналитика", "Analytics", "Análisis", "Analyse")),
        ("profile", ("Профиль", "Profile", "Perfil", "Profil")),
        // Калькулятор
        ("recipe_name", ("Название рецепта", "Recipe name", "Nombre de receta", "Rezeptname")),
        ("category", ("Категория", "Category", "Categoría", "Kategorie")),
        ("no_category", ("Без категории", "No category", "Sin categoría", "Keine Kategorie")),
        ("portions", ("Порций", "Portions", "Porciones", "Portionen")),
        ("weight", ("Вес (кг)", "Weight (kg)", "Peso (kg)", "Gewicht (kg)")),
        ("ingredients", ("Ингредиенты", "Ingredients", "Ingredientes", "Zutaten")),
        ("add", ("+ Добавить", "+ Add", "+ Agregar", "+ Hinzufügen")),
        ("calculate", ("📊 Рассчитать", "📊 Calculate", "📊 Calcular", "📊 Berechnen")),
        ("save", ("💾 Сохранить", "💾 Save", "💾 Guardar", "💾 Speichern")),
        ("additional_costs", ("Доп. расходы", "Extra costs", "Costos extra", "Zusatzkosten")),
        ("results", ("Результаты", "Results", "Resultados", "Ergebnisse")),
        ("cost_price", ("Себестоимость", "Cost price", "Costo", "Selbstkosten")),
        ("profit", ("Чистая прибыль", "Net profit", "Ganancia neta", "Nettogewinn")),
        ("margin", ("Маржа", "Margin", "Margen", "Marge")),
        ("selling_price", ("Цена продажи", "Selling price", "Precio de venta", "Verkaufspreis")),
        ("per_portion", ("За порцию", "Per portion", "Por porción", "Pro Portion")),
        ("roi", ("Рентабельность", "ROI", "ROI", "ROI")),
        ("saved", ("✅ Рецепт сохранён!", "✅ Recipe saved!", "✅ ¡Receta guardada!", "✅ Rezept gespeichert!")),
        // Конвертеры
        ("measures", ("Меры", "Measures", "Medidas", "Maße")),
        ("pounds_ounces", ("Фунты/Унции", "Pounds/Ounces", "Libras/Onzas", "Pfund/Unzen")),
        ("temperature", ("Температура", "Temperature", "Temperatura", "Temperatur")),
        ("eggs", ("Яйца", "Eggs", "Huevos", "Eier")),
        ("sweetener", ("Сахарозаменители", "Sweeteners", "Edulcorantes", "Süßstoffe")),
        ("yeast", ("Дрожжи", "Yeast", "Levadura", "Hefe")),
        ("product", ("Продукт", "Product", "Producto", "Produkt")),
        ("amount", ("Количество", "Amount", "Cantidad", "Menge")),
        ("unit", ("Единица", "Unit", "Unidad", "Einheit")),
        ("convert", ("🔄 Конвертировать", "🔄 Convert", "🔄 Convertir", "🔄 Konvertieren")),
        ("result", ("Результат", "Result", "Resultado", "Ergebnis")),
        ("celsius", ("Цельсий (°C)", "Celsius (°C)", "Celsius (°C)", "Celsius (°C)")),
        ("fahrenheit", ("Фаренгейт (°F)", "Fahrenheit (°F)", "Fahrenheit (°F)", "Fahrenheit (°F)")),
        ("baking_modes", ("Режимы выпечки", "Baking modes", "Modos de horneado", "Backmodi")),
        ("grams", ("г", "g", "g", "g")),
        ("kg", ("кг", "kg", "kg", "kg")),
        ("oz", ("oz (унции)", "oz", "oz", "oz")),
        ("lb", ("lb (фунты)", "lb", "lb", "lb")),
        ("cup", ("чаш. (cup)", "cup", "taza", "Tasse")),
        ("tbsp", ("ст.л.", "tbsp", "cda", "EL")),
        ("tsp", ("ч.л.", "tsp", "cdta", "TL")),
        ("ml", ("мл", "ml", "ml", "ml")),
        ("l", ("л", "l", "l", "l")),
        ("stick_butter", ("стик масла", "butter stick", "barra manteq.", "Butterstück")),
        ("quick_conversions", ("Быстрые конверсии", "Quick conversions", "Conversiones rápidas", "Schnelle Umrechnungen")),
        // Тарифы
        ("free_plan", ("Free", "Free", "Gratis", "Kostenlos")),
        ("pro_plan", ("Pro", "Pro", "Pro", "Pro")),
        ("business_plan", ("Business", "Business", "Negocio", "Business")),
        ("free_price", ("0 ₽", "", "", "0 €")),
        ("pro_price", ("490 ₽/мес", "/mo", "/mes", "5 €/Monat")),
        ("business_price", ("990 ₽/мес", "/mo", "/mes", "10 €/Monat")),
        ("choose", ("Выбрать", "Choose", "Elegir", "Wählen")),
        // Профиль
        ("edit", ("✏️ Редактировать", "✏️ Edit", "✏️ Editar", "✏️ Bearbeiten")),
        ("logout", ("🚪 Выйти", "🚪 Logout", "🚪 Salir", "🚪 Abmelden")),
        ("upgrade", ("⬆️ Улучшить тариф", "⬆️ Upgrade", "⬆️ Mejorar", "⬆️ Upgraden")),
        // Аналитика
        ("search_city", ("🔍 Поиск города", "🔍 Search city", "🔍 Buscar ciudad", "🔍 Stadt suchen")),
        ("prices_in", ("Цены —", "Prices —", "Precios —", "Preise —")),
        ("currency_rate", ("Курс:", "Rate:", "Tasa:", "Kurs:")),
        ("ingredient_name", ("Ингредиент", "Ingredient", "Ingrediente", "Zutat")),
        ("local_price", ("Местная цена", "Local price", "Precio local", "Lokaler Preis")),
        ("in_rubles", ("В рублях", "In RUB", "En RUB", "In RUB")),
    ]);

    match dict.get(key) {
        Some((ru, en, es, de)) => {
            match lang {
                Lang::Ru => ru.to_string(),
                Lang::En => en.to_string(),
                Lang::Es => es.to_string(),
                Lang::De => de.to_string(),
            }
        }
        None => key.to_string(),
    }
}
