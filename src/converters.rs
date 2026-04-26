use crate::models::MeasureUnit;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ProductDensity {
    pub name: String,
    pub density_g_per_ml: f64,
    pub tablespoon_g: f64,
    pub teaspoon_g: f64,
    pub cup_g: f64,
}

impl ProductDensity {
    pub fn database() -> HashMap<String, ProductDensity> {
        let mut db = HashMap::new();
        db.insert("wheat_flour".into(), ProductDensity { name: "Мука пшеничная".into(), density_g_per_ml: 0.59, tablespoon_g: 25.0, teaspoon_g: 10.0, cup_g: 130.0 });
        db.insert("almond_flour".into(), ProductDensity { name: "Мука миндальная".into(), density_g_per_ml: 0.52, tablespoon_g: 18.0, teaspoon_g: 7.0, cup_g: 115.0 });
        db.insert("corn_starch".into(), ProductDensity { name: "Крахмал кукурузный".into(), density_g_per_ml: 0.55, tablespoon_g: 18.0, teaspoon_g: 7.0, cup_g: 130.0 });
        db.insert("sugar_white".into(), ProductDensity { name: "Сахар белый".into(), density_g_per_ml: 0.85, tablespoon_g: 25.0, teaspoon_g: 8.0, cup_g: 200.0 });
        db.insert("powdered_sugar".into(), ProductDensity { name: "Сахарная пудра".into(), density_g_per_ml: 0.55, tablespoon_g: 20.0, teaspoon_g: 7.0, cup_g: 130.0 });
        db.insert("brown_sugar".into(), ProductDensity { name: "Сахар коричневый".into(), density_g_per_ml: 0.82, tablespoon_g: 22.0, teaspoon_g: 7.5, cup_g: 193.0 });
        db.insert("butter".into(), ProductDensity { name: "Масло сливочное".into(), density_g_per_ml: 0.91, tablespoon_g: 17.0, teaspoon_g: 6.0, cup_g: 215.0 });
        db.insert("vegetable_oil".into(), ProductDensity { name: "Масло растительное".into(), density_g_per_ml: 0.92, tablespoon_g: 13.8, teaspoon_g: 4.6, cup_g: 217.0 });
        db.insert("milk_whole".into(), ProductDensity { name: "Молоко цельное".into(), density_g_per_ml: 1.03, tablespoon_g: 15.0, teaspoon_g: 5.0, cup_g: 243.0 });
        db.insert("cream_33".into(), ProductDensity { name: "Сливки 33%".into(), density_g_per_ml: 1.01, tablespoon_g: 15.0, teaspoon_g: 5.0, cup_g: 238.0 });
        db.insert("sour_cream_20".into(), ProductDensity { name: "Сметана 20%".into(), density_g_per_ml: 1.02, tablespoon_g: 18.0, teaspoon_g: 6.0, cup_g: 241.0 });
        db.insert("cream_cheese".into(), ProductDensity { name: "Сливочный сыр".into(), density_g_per_ml: 1.02, tablespoon_g: 20.0, teaspoon_g: 8.0, cup_g: 241.0 });
        db.insert("honey".into(), ProductDensity { name: "Мёд".into(), density_g_per_ml: 1.42, tablespoon_g: 30.0, teaspoon_g: 12.0, cup_g: 335.0 });
        db.insert("cocoa_powder".into(), ProductDensity { name: "Какао-порошок".into(), density_g_per_ml: 0.42, tablespoon_g: 15.0, teaspoon_g: 5.0, cup_g: 100.0 });
        db.insert("salt".into(), ProductDensity { name: "Соль".into(), density_g_per_ml: 1.28, tablespoon_g: 20.0, teaspoon_g: 7.0, cup_g: 302.0 });
        db.insert("water".into(), ProductDensity { name: "Вода".into(), density_g_per_ml: 1.00, tablespoon_g: 15.0, teaspoon_g: 5.0, cup_g: 236.0 });
        db.insert("egg_white".into(), ProductDensity { name: "Яичный белок".into(), density_g_per_ml: 1.04, tablespoon_g: 15.0, teaspoon_g: 5.0, cup_g: 245.0 });
        db.insert("yeast_dry".into(), ProductDensity { name: "Дрожжи сухие".into(), density_g_per_ml: 0.68, tablespoon_g: 10.0, teaspoon_g: 3.3, cup_g: 160.0 });
        db.insert("gelatin_powder".into(), ProductDensity { name: "Желатин порошковый".into(), density_g_per_ml: 0.65, tablespoon_g: 10.0, teaspoon_g: 3.3, cup_g: 153.0 });
        db.insert("baking_powder".into(), ProductDensity { name: "Разрыхлитель".into(), density_g_per_ml: 0.65, tablespoon_g: 12.0, teaspoon_g: 4.0, cup_g: 153.0 });
        db.insert("baking_soda".into(), ProductDensity { name: "Сода пищевая".into(), density_g_per_ml: 0.90, tablespoon_g: 18.0, teaspoon_g: 6.0, cup_g: 212.0 });
        db
    }
}

pub struct MeasureConverter;

#[derive(Clone, Debug, PartialEq)]
pub struct SingleConversion {
    pub amount: f64,
    pub unit_label: String,
}

impl MeasureConverter {
    pub fn convert(product_key: &str, from_amount: f64, from_unit: &MeasureUnit) -> Option<Vec<SingleConversion>> {
        let db = ProductDensity::database();
        let product = db.get(product_key)?;

        // Приводим к граммам
        let amount_in_grams = match from_unit {
            MeasureUnit::Gram => from_amount,
            MeasureUnit::Kilogram => from_amount * 1000.0,
            MeasureUnit::Ounce => from_amount * 28.3495,
            MeasureUnit::Pound => from_amount * 453.592,
            MeasureUnit::StickOfButter => from_amount * 113.4,
            MeasureUnit::Ml => from_amount * product.density_g_per_ml,
            MeasureUnit::Liter => from_amount * 1000.0 * product.density_g_per_ml,
            MeasureUnit::Tablespoon => from_amount * product.tablespoon_g,
            MeasureUnit::Teaspoon => from_amount * product.teaspoon_g,
            MeasureUnit::Cup => from_amount * product.cup_g,
            MeasureUnit::FluidOunce => from_amount * 29.5735,
            MeasureUnit::Pint => from_amount * 473.176,
            MeasureUnit::Quart => from_amount * 946.353,
            MeasureUnit::Gallon => from_amount * 3785.41,
            MeasureUnit::Piece => return Some(vec![SingleConversion { amount: from_amount, unit_label: "шт.".into() }]),
            _ => from_amount,
        };

        let mut conversions = vec![
            SingleConversion { amount: amount_in_grams, unit_label: "г".into() },
            SingleConversion { amount: amount_in_grams / 1000.0, unit_label: "кг".into() },
            SingleConversion { amount: amount_in_grams / 28.3495, unit_label: "oz (унции)".into() },
            SingleConversion { amount: amount_in_grams / 453.592, unit_label: "lb (фунты)".into() },
        ];

        if product.tablespoon_g > 0.0 {
            conversions.push(SingleConversion { amount: amount_in_grams / product.tablespoon_g, unit_label: "ст.л.".into() });
        }
        if product.teaspoon_g > 0.0 {
            conversions.push(SingleConversion { amount: amount_in_grams / product.teaspoon_g, unit_label: "ч.л.".into() });
        }
        if product.cup_g > 0.0 {
            conversions.push(SingleConversion { amount: amount_in_grams / product.cup_g, unit_label: "чаш. (cup)".into() });
        }
        if product.density_g_per_ml > 0.0 {
            let ml = amount_in_grams / product.density_g_per_ml;
            conversions.push(SingleConversion { amount: ml, unit_label: "мл".into() });
            conversions.push(SingleConversion { amount: ml / 1000.0, unit_label: "л".into() });
        }

        if from_unit == &MeasureUnit::StickOfButter || product_key == "butter" {
            conversions.push(SingleConversion { amount: amount_in_grams / 113.4, unit_label: "стик масла".into() });
        }

        Some(conversions)
    }
}

pub struct TemperatureConverter;

impl TemperatureConverter {
    pub fn c_to_f(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
    pub fn f_to_c(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }
}
