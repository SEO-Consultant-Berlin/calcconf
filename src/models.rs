use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecipeCategory {
    Cake, Cupcakes, Macarons, Gingerbread, Mousse,
    Cheesecake, Bread, Pastry, Cookies, Cream, Other(String),
}

impl RecipeCategory {
    pub fn label(&self) -> &str {
        match self {
            RecipeCategory::Cake => "Торт", RecipeCategory::Cupcakes => "Капкейки",
            RecipeCategory::Macarons => "Макаруны", RecipeCategory::Gingerbread => "Пряники",
            RecipeCategory::Mousse => "Муссовый десерт", RecipeCategory::Cheesecake => "Чизкейк",
            RecipeCategory::Bread => "Хлеб", RecipeCategory::Pastry => "Пирожные",
            RecipeCategory::Cookies => "Печенье", RecipeCategory::Cream => "Крем",
            RecipeCategory::Other(s) => s,
        }
    }
    pub fn all() -> Vec<RecipeCategory> {
        vec![RecipeCategory::Cake, RecipeCategory::Cupcakes, RecipeCategory::Macarons,
             RecipeCategory::Gingerbread, RecipeCategory::Mousse, RecipeCategory::Cheesecake,
             RecipeCategory::Bread, RecipeCategory::Pastry, RecipeCategory::Cookies,
             RecipeCategory::Cream]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MeasureUnit {
    Gram, Kilogram, Ounce, Pound, StickOfButter,
    Ml, Liter, Tablespoon, Teaspoon, Cup, FluidOunce, Pint, Quart, Gallon, Pinch, Dash,
    Millimeter, Centimeter, Meter, Inch, Foot,
    Celsius, Fahrenheit,
    Piece,
}

impl MeasureUnit {
    pub fn label(&self) -> &str {
        match self {
            MeasureUnit::Gram => "г", MeasureUnit::Kilogram => "кг",
            MeasureUnit::Ounce => "oz", MeasureUnit::Pound => "lb",
            MeasureUnit::StickOfButter => "стик", MeasureUnit::Ml => "мл",
            MeasureUnit::Liter => "л", MeasureUnit::Tablespoon => "ст.л.",
            MeasureUnit::Teaspoon => "ч.л.", MeasureUnit::Cup => "чаш.",
            MeasureUnit::FluidOunce => "fl oz", MeasureUnit::Pint => "пинта",
            MeasureUnit::Quart => "кварта", MeasureUnit::Gallon => "гал.",
            MeasureUnit::Pinch => "щеп.", MeasureUnit::Dash => "капл.",
            MeasureUnit::Millimeter => "мм", MeasureUnit::Centimeter => "см",
            MeasureUnit::Meter => "м", MeasureUnit::Inch => "in",
            MeasureUnit::Foot => "ft", MeasureUnit::Celsius => "°C",
            MeasureUnit::Fahrenheit => "°F", MeasureUnit::Piece => "шт.",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub price_per_unit: f64,
    pub amount: f64,
    pub unit: MeasureUnit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdditionalCosts {
    pub packaging_percent: f64,
    pub electricity_percent: f64,
    pub amortization_percent: f64,
    pub delivery_fixed: f64,
    pub marketplace_commission: f64,
    pub desired_margin_percent: f64,
    pub tax_percent: f64,
}

impl Default for AdditionalCosts {
    fn default() -> Self {
        AdditionalCosts {
            packaging_percent: 5.0, electricity_percent: 3.0,
            amortization_percent: 2.0, delivery_fixed: 300.0,
            marketplace_commission: 0.0, desired_margin_percent: 100.0,
            tax_percent: 6.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub category: Option<RecipeCategory>,
    pub ingredients: Vec<Ingredient>,
    pub additional: AdditionalCosts,
    pub portions: f64,
    pub portion_unit: String,
    pub total_weight_kg: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalculationResult {
    pub ingredients_cost: f64,
    pub total_cost: f64,
    pub cost_per_portion: f64,
    pub recommended_price: f64,
    pub margin_per_portion: f64,
    pub total_margin: f64,
    pub roi_percent: f64,
    pub tax_amount: f64,
    pub net_profit: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SubscriptionTier {
    Free,
    Trial { ends_at: String },
    Pro,
    Business,
}

impl SubscriptionTier {
    pub fn label(&self) -> &str {
        match self {
            SubscriptionTier::Free => "Free", SubscriptionTier::Trial { .. } => "Trial",
            SubscriptionTier::Pro => "Pro", SubscriptionTier::Business => "Business",
        }
    }
}
