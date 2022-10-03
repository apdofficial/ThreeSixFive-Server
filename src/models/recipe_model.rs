use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeStep {
    pub description: String,
}

impl RecipeStep {

    pub(crate) fn vec_to_string(steps: Vec<RecipeStep>) -> String {
        let mut result = String::new();
        result += "{";
        for step in steps{
            result += "{";
            result += format!("name:{},", step.description.to_owned()).as_str();
            result += "}";
        }
        result += "}";
        result
    }
}

impl Clone for RecipeStep {
    fn clone(&self) -> Self {
        RecipeStep{
            description: self.description.to_owned()
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub title: String
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image{
            path: self.path.to_owned(),
            width: self.width.to_owned(),
            height: self.height.to_owned(),
            title: self.title.to_owned()
        }
    }
}

impl Image {

    pub(crate) fn vec_to_string(images: Vec<Image>) -> String {
        let mut result = String::new();
        result += "{";
        for image in images{
            result += "{";
            result += format!("path:{},", image.path.to_owned()).as_str();
            result += format!("width:{},", image.width.to_owned()).as_str();
            result += format!("height:{},", image.height.to_owned()).as_str();
            result += format!("title:{},", image.title.to_owned()).as_str();
            result += "}";
        }
        result += "}";
        result
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

impl Ingredient {

    pub(crate) fn vec_to_string(ingredients: Vec<Ingredient>) -> String {

        let mut result = String::new();
        result.push_str("{");
        for ingredient in ingredients{
            result.push_str("{");
            result.push_str(format!("name:{},", ingredient.name.to_owned()).as_str());
            result.push_str(format!("unit:{},", ingredient.unit.to_string()).as_str());
            result.push_str(format!("amount:{},", ingredient.amount.to_owned()).as_str());
            result.push_str("}");
        }
        result.push_str("}");
        result
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

impl Recipe {

    pub fn to_doc(&self) -> Document{
        doc! {
            "$set":
                {
                    "id": self.id.to_owned(),
                    "name": self.name.to_owned(),
                    "images": Image::vec_to_string(self.images.to_owned()),
                    "preparation_time_in_minutes": self.preparation_time_in_minutes.to_owned(),
                    "nutrition":
                            {
                                "calories": self.nutrition.calories.to_owned(),
                                "fat": self.nutrition.fat.to_owned(),
                                "carbs": self.nutrition.carbs.to_owned(),
                                "fiber": self.nutrition.fiber.to_owned(),
                                "protein": self.nutrition.protein.to_owned(),
                                "sugars": self.nutrition.sugars.to_owned(),
                                "sodium": self.nutrition.sodium.to_owned(),
                            },
                    "num_of_likes": self.num_of_likes.to_owned(),
                    "num_of_views": self.num_of_views.to_owned(),
                    "ingredients": Ingredient::vec_to_string(self.ingredients.to_owned()),
                    "steps": RecipeStep::vec_to_string(self.steps.to_owned())
                },
        }
    }
}