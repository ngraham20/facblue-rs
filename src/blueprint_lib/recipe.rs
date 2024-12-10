use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Recipes = HashMap<String, Recipe>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields, expecting = "list or empty object")]
pub enum IngredientMeta {
    List(Vec<Ingredient>),
    Empty {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    #[serde(rename = "type")]
    recipe_type: String,
    name: String,
    enabled: Option<bool>,
    ingredients: Option<IngredientMeta>,
    results: Option<IngredientMeta>,
    energy_required: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    #[serde(rename = "type")]
    ingredient_type: IngredientType,
    name: String,
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IngredientType {
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "fluid")]
    Fluid,
}