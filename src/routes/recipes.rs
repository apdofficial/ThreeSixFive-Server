use mongodb::bson::doc;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::models::recipe::Recipe;
use crate::request_guards::basic::ApiKey;
use crate::db::{parse_id, recipe};

use crate::errors::response::MyError;

#[openapi(tag = "Recipe")]
#[post("/recipe", data = "<recipe>")]
pub async fn post_recipe(
    db: &State<Database>,
    recipe: Json<Recipe>,
    _key: ApiKey,
) -> Result<Json<String>, MyError> {
    return match recipe::insert_recipe(&db, recipe.into_inner()).await {
        Ok(result) => {
            Ok(Json(result.inserted_id.to_string()))
        }
        Err(_error) => {
            println!("{:?}", _error);
            Err(MyError::build(
                Status::InternalServerError.code,
                Some(format!("Recipe not added."))
            ))
        }
    }
}

#[openapi(tag = "Recipe")]
#[get("/recipe/<id>")]
pub async fn get_recipe(
    db: &State<Database>,
    id: String,
    _key: ApiKey,
) -> Result<Json<Recipe>, MyError> {
    let id = parse_id(&id)
        .map_err(|err|MyError::build(
            Status::BadRequest.code,
            Some(err.details))
        )?;

    match recipe::find_one_recipe(&db, id).await {
        Ok(recipe) => {
            if recipe.is_none() {
                return Err(MyError::build(
                    Status::NotFound.code,
                    Some(format!("Recipe not found with _id {}", &id)),
                ));
            }
            Ok(Json(recipe.unwrap()))
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(
                Status::NotFound.code,
                Some(format!("Recipe not found with _id {}", &id)),
            ));
        }
    }
}


#[openapi(tag = "Recipe")]
#[put("/recipe/<id>", data = "<recipe>")]
pub async fn update_recipe(
    db: &State<Database>,
    id: String,
    recipe: Json<Recipe>,
    _key: ApiKey,
) -> Result<Json<Recipe>, MyError> {
    let id = parse_id(&id)
        .map_err(|err|MyError::build(
            Status::BadRequest.code,
            Some(err.details))
        )?;
    match recipe::update_recipe(&db, id, recipe.into_inner()).await {
        Ok(recipe) => {
            if recipe.is_none() {
                return Err(MyError::build(
                    Status::NotFound.code,
                    Some(format!("Recipe not found with _id {}", &id)),
                ));
            }
            Ok(Json(recipe.unwrap()))
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(
                Status::BadRequest.code,
                Some(format!("Recipe not found with _id {}", &id)),
            ));
        }
    }
}

#[openapi(tag = "Recipe")]
#[delete("/recipe/<id>")]
pub async fn delete_recipe(
    db: &State<Database>,
    id: String,
    _key: ApiKey,
) -> Result<Json<&str>, MyError> {
    let id = parse_id(&id)
        .map_err(|err|MyError::build(
            Status::BadRequest.code,
            Some(err.details))
        )?;
    return match recipe::delete_one_recipe(&db, id).await {
        Ok(res) => {
            if res.deleted_count == 1 {
                Ok(Json("recipe successfully deleted!"))
            } else {
                Err(MyError::build(
                    Status::NotFound.code,
                    Some("Not Found.".to_string())
                ))
            }
        }
        Err(error) => {
            println!("{:?}", error);
            Err(MyError::build(
                Status::BadRequest.code,
                Some(format!("Recipe not found with _id {}", &id)),
            ))
        }
    }
}

#[openapi(tag = "Recipe")]
#[get("/recipes")]
pub async fn get_all_recipes(
    db: &State<Database>,
    _key: ApiKey
) -> Result<Json<Vec<Recipe>>, MyError> {
    match recipe::find_all_recipes(&db).await {
        Ok(_docs) => Ok(Json(_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(Status::BadRequest.code, Some(_error.to_string())));
        }
    }
}