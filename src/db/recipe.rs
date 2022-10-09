use crate::db::error::DbError;
use crate::db::{create_filter, get_recipes_collection};
use crate::models::recipe::{Recipe, RecipeDocument};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Collection, Database};
use mongodb::bson::Bson::Document;
use rocket::data::N;
use db::crud;
use crate::db;
use crate::models::DocumentConvertable;

pub async fn insert_recipe(db: &Database, recipe: Recipe) -> Result<InsertOneResult, DbError> {
    let collection = get_recipes_collection(&db);
    crud::insert_one(collection, recipe.to_document()).await
}

pub async fn find_one_recipe(db: &Database, id: ObjectId) -> Result<Option<Recipe>, DbError> {
    let collection = get_recipes_collection(&db);
    crud::find_one(collection, id).await
}

pub async fn find_all_recipes(db: &Database) -> Result<Vec<Recipe>, DbError> {
    let collection = get_recipes_collection(&db);
    crud::find_all(collection).await
}

pub async fn update_recipe(
    db: &Database,
    id: ObjectId,
    recipe: Recipe,
) -> Result<Option<Recipe>, DbError> {
    let collection = get_recipes_collection(&db);
    crud::update_one(collection, id, recipe.to_document()).await
}

pub async fn delete_one_recipe(db: &Database, id: ObjectId) -> Result<DeleteResult, DbError> {
    let collection = get_recipes_collection(&db);
    crud::delete_one(collection, id).await
}
