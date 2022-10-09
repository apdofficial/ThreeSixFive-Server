use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};
use rocket::fairing::AdHoc;
use std::env;
use dotenv::Error;
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use crate::db::error::DbError;
use crate::models::image::{Image, ImageDocument, ImageFile};
use crate::models::recipe::{Recipe, RecipeDocument};
use crate::models::gif::RecipeStepDocument;

pub mod customer;
pub mod recipe;
pub mod error;
pub mod image;
pub mod gif;
pub mod crud;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(database),
            Err(error) => {
                panic!("Cannot connect to instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI is not found.");
    let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not found.");

    let client = Client::with_uri_str(mongo_uri).await?;
    let database = client.database(mongo_db_name.as_str());

    println!("MongoDB Connected!");

    Ok(database)
}

fn get_recipe_steps_collection(db: & Database) -> Collection<RecipeStepDocument> {
    db.collection::<RecipeStepDocument>("RecipeSteps")
}

fn get_recipes_collection(db: & Database) -> Collection<RecipeDocument> {
    db.collection::<RecipeDocument>("Recipes")
}

fn get_images_collection(db: & Database) -> Collection<ImageDocument> {
    db.collection::<ImageDocument>("Images")
}

fn create_filter(id: & ObjectId) -> Result<Document, DbError> {
    Ok(doc! { "_id": id })
}

pub fn parse_id(id: &String) -> Result<ObjectId, DbError>{
    if id.is_empty() {
        return Err(DbError::new(format!("failed to parse {} as an ObjectId.", id)))
    };

    match ObjectId::parse_str(id) {
        Ok(id) => Ok(id),
        Err(error) => Err(DbError::new(error.to_string()))
    }
}


