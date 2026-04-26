use crate::models::Recipe;
use gloo::storage::{LocalStorage, Storage};

const RECIPES_KEY: &str = "confectioner_recipes";

pub struct AppStorage;

impl AppStorage {
    pub fn load_recipes() -> Vec<Recipe> {
        LocalStorage::get(RECIPES_KEY).unwrap_or_default()
    }

    pub fn save_recipes(recipes: &Vec<Recipe>) {
        LocalStorage::set(RECIPES_KEY, recipes).ok();
    }

    pub fn save_recipe(recipe: &Recipe) {
        let mut recipes = Self::load_recipes();
        if let Some(pos) = recipes.iter().position(|r| r.id == recipe.id) {
            recipes[pos] = recipe.clone();
        } else {
            recipes.push(recipe.clone());
        }
        Self::save_recipes(&recipes);
    }

    pub fn delete_recipe(id: &str) {
        let mut recipes = Self::load_recipes();
        recipes.retain(|r| r.id != id);
        Self::save_recipes(&recipes);
    }

    pub fn get_recipe_count() -> usize {
        Self::load_recipes().len()
    }
}
