use crate::{
    models::recipe_model::Recipe,
    repository::mongodb_repo::MongoRepo
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};


#[post("/recipe", data = "<new_recipe>")]
pub fn create_recipe(
    db: &State<MongoRepo>,
    new_recipe: Json<Recipe>,
) -> Result<Json<InsertOneResult>, Status> {
    let recipe_detail = db.create_recipe(new_recipe.into_inner());
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipe/<path>")]
pub fn get_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<Recipe>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let recipe_detail = db.get_recipe(&id);

    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/recipe/<path>", data = "<new_recipe>")]
pub fn update_recipe(
    db: &State<MongoRepo>,
    path: String,
    new_recipe: Json<Recipe>,
) -> Result<Json<Recipe>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let mut data = new_recipe.into_inner();
    data.id = Some(id.clone());

    let update_result = db.update_recipe(&id, data);

    match update_result {
        Ok(update) => {
            return if update.matched_count == 1 {
                let updated_recipe_info = db.get_recipe(&id);

                match updated_recipe_info {
                    Ok(recipe) => Ok(Json(recipe)),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/recipe/<path>")]
pub fn delete_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let result = db.delete_recipe(&id);

    match result {
        Ok(res) => {
            return if res.deleted_count == 1 {
                Ok(Json("recipe successfully deleted!"))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipes")]
pub fn get_all_recipes(db: &State<MongoRepo>) -> Result<Json<Vec<Recipe>>, Status> {
    let recipes = db.get_all_recipes();

    match recipes {
        Ok(recipes) => Ok(Json(recipes)),
        Err(_) => Err(Status::InternalServerError),
    }
}
