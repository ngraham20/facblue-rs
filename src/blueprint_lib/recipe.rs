use anyhow::{anyhow, bail, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields, expecting = "list or empty object")]
pub enum IngredientMeta {
    List(Vec<Ingredient>),
    Empty {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    #[serde(rename = "type")]
    pub recipe_type: String,
    pub name: String,
    enabled: Option<bool>,
    pub ingredients: Option<IngredientMeta>,
    pub results: Option<IngredientMeta>,
    pub energy_required: Option<f64>,
}

impl Recipe {
    pub fn craft_count(&self) -> Option<f64> {
        if let Some(item) = self.desired_item() {
            return Some(item.amount)
        }
        None
    }

    pub fn desired_item(&self) -> Option<Ingredient> {
        if let Some(meta) = &self.results {
            if let IngredientMeta::List(ingredients) = meta {
                for ingredient in ingredients {
                    if ingredient.name == self.name {
                        return Some(ingredient.clone())
                    }
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingredient {
    #[serde(rename = "type")]
    pub ingredient_type: IngredientType,
    pub name: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IngredientType {
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "fluid")]
    Fluid,
}

impl ToString for IngredientType {
    fn to_string(&self) -> String {
        match self {
            IngredientType::Item => String::from("item"),
            IngredientType::Fluid => String::from("fluid")
        }
    }
}