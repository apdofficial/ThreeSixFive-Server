use crate::models::image_model::Image;
use crate::models::recipe_model::{Ingredient, IngredientUnit, Nutrition, Recipe, RecipeStep};

pub fn get_recipe() -> Recipe{
    Recipe {
        id: None,
        name: "Salmon".parse().unwrap(),
        images: vec![
            Image {
                id: None,
                path: "1".to_string(),
                width: 10,
                height: 10,
                title: "hello 1".to_string()
            },
            Image {
                id: None,
                path: "2".to_string(),
                width: 10,
                height: 10,
                title: "hello 1".to_string()
            }
        ],
        preparation_time_in_minutes: 20,
        nutrition: Nutrition {
            calories: 10,
            fat: 20,
            carbs: 5,
            fiber: 45,
            protein: 12,
            sugars: 1,
            sodium: 2
        },
        num_of_views: 0,
        num_of_likes: 0,
        ingredients: vec! [
            Ingredient {
                name: "Salmon".parse().unwrap(),
                amount: 250,
                unit: IngredientUnit::g
            },
            Ingredient {
                name: "Potatos".parse().unwrap(),
                amount: 125,
                unit: IngredientUnit::g
            },
            Ingredient {
                name: "Salt".parse().unwrap(),
                amount: 5,
                unit: IngredientUnit::g
            }
        ],
        steps: vec![
            RecipeStep{
                description: "Slice the salmon.".to_string()
            },
            RecipeStep{
                description: "Cook it with the potatoes.".to_string()
            },
            RecipeStep{
                description: "Enjoy.".to_string()
            }
        ]
    }
}