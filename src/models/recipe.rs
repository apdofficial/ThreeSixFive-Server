use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;
use crate::models::image::Image;
use crate::models::gif::RecipeStep;
use crate::models::{DocumentConvertable, ObjectConvertable};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub images: Vec<Image>,
    pub preparation_time_in_minutes: i32,
    pub nutrition: Nutrition,
    pub num_of_likes: i32,
    pub num_of_views: i32,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<RecipeStep>,
    pub created_at: DateTime,
}

impl ObjectConvertable<Recipe> for RecipeDocument{
    fn to_object(&self) -> Recipe {
        Recipe{
            _id: self._id.clone().unwrap_or(ObjectId::new()).to_string(),
            name: self.name.clone(),
            images: self.images.clone(),
            preparation_time_in_minutes: self.preparation_time_in_minutes,
            nutrition: Nutrition {
                calories: self.nutrition.calories,
                fat: self.nutrition.fat,
                carbs: self.nutrition.carbs,
                fiber: self.nutrition.fiber,
                protein: self.nutrition.protein,
            },
            num_of_likes: self.num_of_likes,
            num_of_views: self.num_of_views,
            ingredients: self.ingredients.clone(),
            steps: self.steps.clone(),
            created_at: self.created_at.to_string()
        }
    }
}


#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Recipe {
    pub _id: String,
    pub name: String,
    pub images: Vec<Image>,
    pub preparation_time_in_minutes: i32,
    pub nutrition: Nutrition,
    pub num_of_likes: i32,
    pub num_of_views: i32,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<RecipeStep>,
    pub created_at: String,
}


impl DocumentConvertable<RecipeDocument> for Recipe{
    fn to_document(&self) -> RecipeDocument {
        RecipeDocument {
            _id: None,
            name: self.name.clone(),
            images: self.images.clone(),
            preparation_time_in_minutes: self.preparation_time_in_minutes,
            nutrition: Nutrition {
                calories: self.nutrition.calories,
                fat: self.nutrition.fat,
                carbs: self.nutrition.carbs,
                fiber: self.nutrition.fiber,
                protein: self.nutrition.protein,
            },
            num_of_likes: self.num_of_likes,
            num_of_views: self.num_of_views,
            ingredients: self.ingredients.clone(),
            steps: self.steps.clone(),
            created_at: DateTime::now()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Nutrition {
    pub calories: i32,
    pub fat: i32,
    pub carbs: i32,
    pub fiber: i32,
    pub protein: i32,
}

#[derive(Debug, Display, PartialEq, EnumString, Serialize, Deserialize, JsonSchema, Clone)]
#[allow(non_camel_case_types)]
pub enum IngredientUnit{
    kg,
    g,
    mg,
    l,
    dcl,
    ml,
    tsp,
    tbsp
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amount: i32,
    pub unit: IngredientUnit,
}