use futures::TryStreamExt;
use crate::db::error::DbError;
use crate::db::{create_filter, get_images_collection, get_recipes_collection};
use crate::models::image::Image;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Collection, Database};
use crate::models::recipe::Recipe;

pub async fn insert_image(db: &Database, image: Image) -> Result<InsertOneResult, DbError> {
    let collection = get_images_collection(&db);
    collection
        .insert_one(image.to_document(), None)
        .await
        .map_err(|_err| DbError::new("Failed to insert_image.".to_string()))
}

pub async fn find_one_image(db: &Database, id: ObjectId) -> Result<Option<Image>, DbError> {
    let collection = get_images_collection(&db);
    let filter = create_filter(&id)?;
    collection
        .find_one(filter, None)
        .await
        .map(|iamge_doc|
            match iamge_doc {
                Some(iamge_doc) => Some(iamge_doc.to_object()),
                None => None
            }
        )
        .map_err(|_err| DbError::new("Failed to find_one_image.".to_string()))
}

pub async fn find_all_images(db: &Database) -> Result<Vec<Image>, DbError> {
    let collection = get_images_collection(&db);
    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|_err| DbError::new("Failed to find_all_images.".to_string()))?;
    let mut images: Vec<Image> = vec![];

    while let candidate = cursor.try_next().await {
        let image = candidate
            .map_err(|_err| DbError::new("Failed to get recipe.".to_string()))?;
        if let Some(image) = image{
            images.push(image.to_object());
        }
    }
    Ok(images)
}

pub async fn delete_one_image(db: &Database, id: ObjectId) -> Result<DeleteResult, DbError> {
    let collection = get_images_collection(&db);
    let filter = create_filter(&id)?;
    collection
        .delete_one(filter, None)
        .await
        .map_err(|_err| DbError::new("Failed to delete_one_image.".to_string()))
}
