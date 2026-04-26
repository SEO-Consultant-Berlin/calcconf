use crate::models::*;
use uuid::Uuid;
use chrono::Utc;

pub struct CostCalculator;

impl CostCalculator {
    pub fn calculate(recipe: &Recipe) -> CalculationResult {
        let ingredients_cost: f64 = recipe.ingredients.iter()
            .map(|ing| {
                let price_per_base = match ing.unit {
                    MeasureUnit::Gram => ing.price_per_unit / 1000.0,
                    MeasureUnit::Kilogram => ing.price_per_unit,
                    MeasureUnit::Ml => ing.price_per_unit / 1000.0,
                    MeasureUnit::Liter => ing.price_per_unit,
                    MeasureUnit::Piece => ing.price_per_unit,
                    MeasureUnit::Tablespoon => ing.price_per_unit / (1000.0 / 15.0),
                    MeasureUnit::Teaspoon => ing.price_per_unit / (1000.0 / 5.0),
                    _ => ing.price_per_unit / 1000.0,
                };
                let amount_in_base = match ing.unit {
                    MeasureUnit::Gram => ing.amount,
                    MeasureUnit::Kilogram => ing.amount * 1000.0,
                    MeasureUnit::Ml => ing.amount,
                    MeasureUnit::Liter => ing.amount * 1000.0,
                    MeasureUnit::Tablespoon => ing.amount * 15.0,
                    MeasureUnit::Teaspoon => ing.amount * 5.0,
                    MeasureUnit::Piece => ing.amount,
                    MeasureUnit::Cup => ing.amount * 236.6,
                    _ => ing.amount,
                };
                price_per_base * amount_in_base
            }).sum();

        let mut total = ingredients_cost;
        total *= 1.0 + recipe.additional.packaging_percent / 100.0;
        total *= 1.0 + recipe.additional.electricity_percent / 100.0;
        total *= 1.0 + recipe.additional.amortization_percent / 100.0;
        total += recipe.additional.delivery_fixed;
        total *= 1.0 + recipe.additional.marketplace_commission / 100.0;

        let cost_per_portion = if recipe.portions > 0.0 { total / recipe.portions } else { total };
        let recommended_price = cost_per_portion * (1.0 + recipe.additional.desired_margin_percent / 100.0);
        let margin_per_portion = recommended_price - cost_per_portion;
        let total_margin = margin_per_portion * recipe.portions;
        let tax_amount = total_margin * recipe.additional.tax_percent / 100.0;
        let net_profit = total_margin - tax_amount;
        let roi_percent = if total > 0.0 { (total_margin / total) * 100.0 } else { 0.0 };

        CalculationResult {
            ingredients_cost, total_cost: total, cost_per_portion,
            recommended_price, margin_per_portion, total_margin,
            roi_percent, tax_amount, net_profit,
        }
    }

    pub fn new_recipe(name: String) -> Recipe {
        let now = Utc::now().to_rfc3339();
        Recipe {
            id: Uuid::new_v4().to_string(),
            name,
            category: None,
            ingredients: vec![
                Ingredient {
                    id: Uuid::new_v4().to_string(),
                    name: "Мука пшеничная".into(),
                    price_per_unit: 80.0,
                    amount: 500.0,
                    unit: MeasureUnit::Gram,
                },
                Ingredient {
                    id: Uuid::new_v4().to_string(),
                    name: "Сахар".into(),
                    price_per_unit: 60.0,
                    amount: 300.0,
                    unit: MeasureUnit::Gram,
                },
                Ingredient {
                    id: Uuid::new_v4().to_string(),
                    name: "Яйца С1".into(),
                    price_per_unit: 12.0,
                    amount: 4.0,
                    unit: MeasureUnit::Piece,
                },
            ],
            additional: AdditionalCosts::default(),
            portions: 8.0,
            portion_unit: "порций".into(),
            total_weight_kg: 1.5,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
