#[macro_use] extern crate rocket;

mod api;
mod models;
mod repository;

use rocket::serde::json::serde_json;
use api::user_api::{create_user, delete_user, get_user, update_user, get_all_users};
use api::recipe_api::{
    post_recipe, 
    post_image,
    delete_recipe, 
    get_recipe, 
    update_recipe, 
    get_all_recipes, 
    get_image,
    delete_image
};
use repository::mongodb_repo::MongoRepo;
use crate::models::recipe_model::{Image, Ingredient, IngredientUnit, Nutrition, Recipe, RecipeStep};


#[launch]
fn rocket() -> _ {
    let dummy_recipe = Recipe {
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
    };

    println!("{}", serde_json::to_string(&dummy_recipe).unwrap());

    let db = MongoRepo::init();
    rocket::build()
        .manage(db)

        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])

        .mount("/", routes![post_recipe])
        .mount("/", routes![delete_recipe])
        .mount("/", routes![get_recipe])
        .mount("/", routes![update_recipe])
        .mount("/", routes![get_all_recipes])

        .mount("/", routes![post_image])
        .mount("/", routes![get_image])
        .mount("/", routes![delete_image])

}