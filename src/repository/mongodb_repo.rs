use std::env;
extern crate dotenv;

use dotenv::dotenv;

use mongodb::{bson::{doc, extjson::de::Error, oid::ObjectId}, bson, results::{InsertOneResult, UpdateResult, DeleteResult}, sync::{Client, Collection}};
use mongodb::bson::extjson::de::Error::DeserializationError;
use mongodb::bson::{Bson, SerializerOptions, to_document};
use crate::IngredientUnit;

use crate::models::user_model::User;
use crate::models::recipe_model::Recipe;

pub struct MongoRepo {
    users: Collection<User>,
    recipes: Collection<Recipe>,
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
        let users: Collection<User> = db.collection("User");
        let recipes: Collection<Recipe> = db.collection("Recipe");
        MongoRepo { users, recipes }
    }
}

/// User DB
impl MongoRepo {
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
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


    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
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

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .users
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }
}

/// Recipe DB
impl MongoRepo {

    pub fn create_recipe(&self, new_recipe: Recipe) -> Result<InsertOneResult, Error> {
        let recipe = self
            .recipes
            .insert_one(new_recipe, None)
            .ok()
            .expect("Error creating recipe");

        Ok(recipe)
    }


    pub fn get_recipe(&self, id: &ObjectId) -> Result<Recipe, Error> {
        let recipe_detail = self
            .recipes
            .find_one(doc! {"_id": id}, None)
            .ok()
            .expect("Error getting recipe's detail");
        Ok(recipe_detail.unwrap())
    }

    pub fn update_recipe(&self, id: &ObjectId, recipe: Recipe) -> Result<UpdateResult, Error> {
        let doc = to_document(&recipe);
        let updated_doc = self
            .recipes
            .update_one(doc! {"_id": id}, doc, None)
            .ok()
            .expect("Error updating recipe");

        Ok(updated_doc)
    }

    pub fn delete_recipe(&self, id: &ObjectId) -> Result<DeleteResult, Error> {
        let recipe_detail = self
            .recipes
            .delete_one(doc! {"_id": id}, None)
            .ok()
            .expect("Error deleting recipe");

        Ok(recipe_detail)
    }

    pub fn get_all_recipes(&self) -> Result<Vec<Recipe>, Error> {
        let cursors = self
            .recipes
            .find(None, None)
            .ok()
            .expect("Error getting list of recipes");
        let recipes = cursors.map(|doc| doc.unwrap()).collect();

        Ok(recipes)
    }
}