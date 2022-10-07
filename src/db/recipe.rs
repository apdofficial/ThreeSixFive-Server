use crate::db::error::DbError;
use crate::db::{create_filter, get_recipes_collection};
use crate::models::recipe::Recipe;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Collection, Database};
use rocket::data::N;

pub async fn insert_recipe(db: &Database, recipe: Recipe) -> Result<InsertOneResult, DbError> {
    let collection = get_recipes_collection(&db);
    collection
        .insert_one(recipe.to_document(), None)
        .await
        .map_err(|_err| DbError::new("Failed to insert_recipe.".to_string()))
}

pub async fn find_one_recipe(db: &Database, id: ObjectId) -> Result<Option<Recipe>, DbError> {
    let collection = get_recipes_collection(&db);
    let filter = create_filter(&id)?;
    collection
        .find_one(filter, None)
        .await
        .map(|recipe_doc|
            match recipe_doc {
                Some(recipe_doc) => Some(recipe_doc.to_object()),
                None => None
            }
        )
        .map_err(|_err| DbError::new("Failed to find_one_recipe.".to_string()))
}

pub async fn find_all_recipes(db: &Database) -> Result<Vec<Recipe>, DbError> {
    let collection = get_recipes_collection(&db);
    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|_err|
            DbError::new(format!("Failed to find_all_recipes:").to_string())
        )?;

    let recipe_documents: Vec<_> = cursor.try_collect().await
        .map_err(|_err|
            DbError::new(format!("Failed to find_all_recipes:").to_string())
        )?;

    let recipes = recipe_documents
        .into_iter()
        .map(|doc| doc.to_object())
        .collect();

    Ok(recipes)
}

pub async fn update_recipe(
    db: &Database,
    id: ObjectId,
    recipe: Recipe,
) -> Result<Option<Recipe>, DbError> {
    let collection = get_recipes_collection(&db);
    let filter = create_filter(&id)?;
    collection
        .find_one_and_replace(filter, recipe.to_document(), None)
        .await
        .map(|recipe_doc|
            match recipe_doc {
                Some(recipe_doc) => Some(recipe_doc.to_object()),
                None => None
            }
        )
        .map_err(|_err| DbError::new("Failed to update_recipe.".to_string()))
}

pub async fn delete_one_recipe(db: &Database, id: ObjectId) -> Result<DeleteResult, DbError> {
    let collection = get_recipes_collection(&db);
    let filter = create_filter(&id)?;
    collection
        .delete_one(filter, None)
        .await
        .map_err(|_err| DbError::new("Failed to delete_one_recipe.".to_string()))
}
