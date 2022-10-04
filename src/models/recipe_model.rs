use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use strum_macros::EnumString;
use crate::models::image_model::Image;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeStep {
    pub description: String,
}

impl Clone for RecipeStep {
    fn clone(&self) -> Self {
        RecipeStep{
            description: self.description.to_owned()
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Nutrition {
    pub calories: i32,
    pub fat: i32,
    pub carbs: i32,
    pub fiber: i32,
    pub protein: i32,
    pub sugars: i32,
    pub sodium: i32
}

#[derive(Debug, Display, PartialEq, EnumString, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_camel_case_types)]
pub enum IngredientUnit{
    kg,
    mg,
    l,
    ml,
    cup,
    g
}

impl Clone for IngredientUnit {
    fn clone(&self) -> IngredientUnit {
        match self {
            IngredientUnit::kg => IngredientUnit::kg,
            IngredientUnit::mg => IngredientUnit::mg,
            IngredientUnit::l => IngredientUnit::l,
            IngredientUnit::ml => IngredientUnit::ml,
            IngredientUnit::cup => IngredientUnit::cup,
            IngredientUnit::g => IngredientUnit::g
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub name: String,
    pub amount: i32,
    pub unit: IngredientUnit,
}

impl Clone for Ingredient {
    fn clone(&self) -> Self {
        Ingredient{
            name: self.name.to_owned(),
            amount: self.amount.to_owned(),
            unit: self.unit.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub images: Vec<Image>,
    pub preparation_time_in_minutes: i32,
    pub nutrition: Nutrition,
    pub num_of_likes: i32,
    pub num_of_views: i32,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<RecipeStep>
}