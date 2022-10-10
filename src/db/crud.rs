use crate::models::ObjectConvertable;
use futures::{TryStream, TryStreamExt};
use mongodb::results::InsertOneResult;
use mongodb::{bson::oid::ObjectId, results::DeleteResult, Collection, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;

use super::{create_filter, error::DbError, get_recipe_steps_collection};

pub async fn insert_one<T, U>(
    collection: Collection<T>,
    doc: impl Borrow<U> + Borrow<T>,
) -> Result<InsertOneResult, DbError>
where
    T: Serialize,
{
    collection
        .insert_one(doc, None)
        .await
        .map_err(|_err| DbError::new("Failed to insert_image.".to_string()))
}

pub async fn find_one<T, U>(collection: Collection<T>, id: ObjectId) -> Result<Option<U>, DbError>
where
    T: DeserializeOwned + Unpin + Send + Sync + ObjectConvertable<U>,
{
    let filter = create_filter(&id)?;
    collection
        .find_one(filter, None)
        .await
        .map(|doc| match doc {
            Some(doc) => Some(doc.to_object()),
            None => None,
        })
        .map_err(|_err| DbError::new("Failed to find_one.".to_string()))
}

trait DBStream: TryStream + TryStreamExt {}

pub async fn find_all<T, U>(collection: Collection<T>) -> Result<Vec<U>, DbError>
where
    T: DeserializeOwned + Unpin + Send + Sync + ObjectConvertable<U>,
{
    let cursor = collection
        .find(None, None)
        .await
        .map_err(|_err| DbError::new("Failed to find_all_images.".to_string()))?;

    let object_documents: Vec<T> = cursor
        .try_collect()
        .await
        .map_err(|_err| DbError::new(format!("Failed to find_all_recipes:").to_string()))?;

    let objects = object_documents
        .into_iter()
        .map(|doc| doc.to_object())
        .collect();

    Ok(objects)
}

pub async fn update_one<T, U>(
    collection: Collection<T>,
    id: ObjectId,
    doc: impl Borrow<T>,
) -> Result<Option<U>, DbError>
where
    T: Serialize + DeserializeOwned + ObjectConvertable<U>,
{
    let filter = create_filter(&id)?;
    collection
        .find_one_and_replace(filter, doc, None)
        .await
        .map(|doc| match doc {
            Some(doc) => Some(doc.to_object()),
            None => None,
        })
        .map_err(|_err| DbError::new("Failed to update_one.".to_string()))
}

pub async fn delete_one<T>(
    collection: Collection<T>,
    id: ObjectId,
) -> Result<DeleteResult, DbError> {
    let filter = create_filter(&id)?;
    collection
        .delete_one(filter, None)
        .await
        .map_err(|_err| DbError::new("Failed to delete_one_recipe.".to_string()))
}
