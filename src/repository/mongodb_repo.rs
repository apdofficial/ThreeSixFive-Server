use std::{env, fmt};
use std::error::Error;
use dotenv::dotenv;
use mongodb::{
    bson::{doc,  oid::ObjectId}, results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection}
};
use crate::models::image_model::Image;
use crate::models::user_model::User;
use crate::models::recipe_model::Recipe;
use crate::repository::db_error::DbError;

pub struct MongoRepo {
    users: Collection<User>,
    recipes: Collection<Recipe>,
    images: Collection<Image>,
}

/// DB
impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("ThreeSixFive");
        let users: Collection<User> = db.collection("Users");
        let recipes: Collection<Recipe> = db.collection("Recipes");
        let images: Collection<Image> = db.collection("Images");
        MongoRepo { users, recipes, images }
    }
}

/// User DB
impl MongoRepo {
    pub fn create_user(&self, new_user: User) -> mongodb::error::Result<InsertOneResult> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .users
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");

        Ok(user)
    }


    pub fn get_user(&self, id: &String) -> mongodb::error::Result<User> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> mongodb::error::Result<UpdateResult> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .users
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> mongodb::error::Result<DeleteResult> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> mongodb::error::Result<Vec<User>> {
        let cursors = self
            .users
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }
}

/// Helpers
impl MongoRepo {

    fn parse_id(id: &String) -> Result<ObjectId, DbError>{
        if id.is_empty() {
            return Err(DbError::new(format!("failed to parse {} as an ObjectId.", id)))
        };
        match ObjectId::parse_str(id) {
            Ok(id) => Ok(id),
            Err(error) => Err(DbError::new(error.to_string()))
        }
    }
}

/// Recipe DB
impl MongoRepo {

    pub async fn create_recipe(&self, new_recipe: Recipe) -> Result<InsertOneResult, DbError> {
        self.recipes.insert_one(new_recipe, None)
            .map_err(|_err| DbError::new("Failed to create_recipe.".to_string()))
    }

    pub async fn get_recipe(&self, id: &String) -> Result<Option<Recipe>, DbError> {
        let id = MongoRepo::parse_id(&id)?;
        self.recipes.find_one(doc! {"_id": id}, None)
            .map_err(|_err| DbError::new("Failed to get_recipe.".to_string()))
    }

    pub async fn update_recipe(&self, id: &String, recipe: Recipe) -> Result<Option<Recipe>, DbError> {
        let id = MongoRepo::parse_id(&id)?;
        self.recipes.find_one_and_replace(doc! {"_id": id}, recipe, None)
             .map_err(|_err| DbError::new("Failed to update_recipe.".to_string()))
    }

    pub async fn delete_recipe(&self, id: &String) -> Result<DeleteResult, DbError> {
        let id = MongoRepo::parse_id(&id)?;
        self.recipes.delete_one(doc! {"_id": id}, None)
            .map_err(|_err| DbError::new("Failed to delete_recipe.".to_string()))
    }

    pub async fn get_all_recipes(&self) -> Result<Vec<Recipe>, DbError> {
        let cursor = self.recipes.find(None, None)
            .map_err(|_err| DbError::new("Failed to get_all_recipes.".to_string()))?;
        let mut recipes: Vec<Recipe> = vec![];
        for recipe in cursor {
            if let Ok(recipe) = recipe {
                recipes.push(recipe);
            }
        }
        Ok(recipes)
    }
}

/// Image DB
impl MongoRepo {

    pub async fn create_image(&self, image: &Image) -> Result<InsertOneResult, DbError> {
        self.images.insert_one(image, None)
            .map_err(|_err| DbError::new("Failed to create_image.".to_string()))
    }

    pub async fn get_image(&self, id: &String) -> Result<Option<Image>, DbError> {
        let id = MongoRepo::parse_id(&id)?;
        self.images.find_one(doc! {"_id": id}, None)
            .map_err(|_err| DbError::new("Failed to get_image.".to_string()))
    }

    pub async fn delete_image(&self, id: &String) -> Result<DeleteResult, DbError> {
        let id = MongoRepo::parse_id(&id)?;
        self.images.delete_one(doc! {"_id": id}, None)
            .map_err(|_err| DbError::new("Failed to delete_image.".to_string()))
    }
}