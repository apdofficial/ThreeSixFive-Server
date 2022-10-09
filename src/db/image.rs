use futures::TryStreamExt;
use crate::db::error::DbError;
use crate::db::{create_filter, crud, get_images_collection, get_recipes_collection};
use crate::models::image::Image;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Collection, Database};
use crate::models::DocumentConvertable;
use crate::models::recipe::Recipe;

pub async fn insert_image(db: &Database, image: Image) -> Result<InsertOneResult, DbError> {
    let collection = get_images_collection(&db);
    crud::insert_one(collection, image.to_document()).await
}

pub async fn find_one_image(db: &Database, id: ObjectId) -> Result<Option<Image>, DbError> {
    let collection = get_images_collection(&db);
    crud::find_one(collection, id).await
}

pub async fn find_all_images(db: &Database) -> Result<Vec<Image>, DbError> {
    let collection = get_images_collection(&db);
    crud::find_all(collection).await
}

pub async fn delete_one_image(db: &Database, id: ObjectId) -> Result<DeleteResult, DbError> {
    let collection = get_images_collection(&db);
    crud::delete_one(collection, id).await
}
