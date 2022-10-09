use crate::db::error::DbError;
use crate::db::{create_filter, crud, get_recipe_steps_collection, get_recipes_collection};
use crate::models::recipe::Recipe;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Collection, Database};
use rocket::data::N;
use crate::models::{DocumentConvertable, ObjectConvertable};
use crate::models::gif::RecipeStep;

pub async fn insert_recipe_step(
    db: &Database,
    recipe_step: RecipeStep,
) -> Result<InsertOneResult, DbError> {
    let collection = get_recipe_steps_collection(&db);
    crud::insert_one(collection, recipe_step.to_document()).await
}

pub async fn find_one_recipe_step(
    db: &Database,
    id: ObjectId,
) -> Result<Option<RecipeStep>, DbError> {
    let collection = get_recipe_steps_collection(&db);
    crud::find_one(collection, id).await
}

pub async fn find_all_recipe_steps(db: &Database) -> Result<Vec<RecipeStep>, DbError> {
    let collection = get_recipe_steps_collection(&db);
    crud::find_all(collection).await
}

pub async fn update_recipe_step(
    db: &Database,
    id: ObjectId,
    recipe_step: RecipeStep,
) -> Result<Option<RecipeStep>, DbError> {
    let collection = get_recipe_steps_collection(&db);
    crud::update_one(collection, id, recipe_step.to_document()).await
}

pub async fn delete_one_recipe_step(db: &Database, id: ObjectId) -> Result<DeleteResult, DbError> {
    let collection = get_recipe_steps_collection(&db);
    crud::delete_one(collection, id).await
}