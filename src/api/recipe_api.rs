use crate::{models::recipe_model::Recipe, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, Response, serde::json::Json, State};


#[post("/recipe", data = "<new_recipe>")]
pub async fn create_recipe(
    db: &State<MongoRepo>,
    new_recipe: Json<Recipe>
) -> Result<Json<InsertOneResult>, Status> {
    match db.create_recipe(new_recipe.into_inner()).await {
        Ok(result) => Ok(Json(result)),
        _ => Err(Status::InternalServerError),
    }
}

#[get("/recipe/<id>")]
pub async fn get_recipe(db: &State<MongoRepo>, id: String) -> Result<Json<Recipe>, Status> {
    match db.get_recipe(&id).await {
        Ok(Some(recipe)) => Ok(Json(recipe)),
        _ => Err(Status::InternalServerError),
    }
}

#[put("/recipe/<id>", data = "<new_recipe>")]
pub async fn update_recipe(
    db: &State<MongoRepo>,
    id: String,
    new_recipe: Json<Recipe>,
) -> Result<Json<Recipe>, Status> {
    let mut data = new_recipe.into_inner();
    data.id = None;
    match db.update_recipe(&id, data).await {
        Ok(Some(recipes)) => Ok(Json(recipes)),
        _ => Err(Status::InternalServerError),
    }
}

#[delete("/recipe/<id>")]
pub async fn delete_recipe(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    match db.delete_recipe(&id).await {
        Ok(res) => {
            return if res.deleted_count == 1 {
                Ok(Json("recipe successfully deleted!"))
            } else {
                Err(Status::NotFound)
            }
        }
        _ => Err(Status::InternalServerError),
    }
}

#[get("/recipes")]
pub async fn get_all_recipes(db: &State<MongoRepo>) -> Result<Json<Vec<Recipe>>, Status> {
    match db.get_all_recipes().await {
        Ok(recipes) => Ok(Json(recipes)),
        _ => Err(Status::InternalServerError),
    }
}