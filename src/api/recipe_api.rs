use std::path::PathBuf;
use crate::{
    models::recipe_model::Recipe,
    repository::mongodb_repo::MongoRepo
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};
use rocket::form::Form;
use rocket::fs::TempFile;
use tokio::io::AsyncReadExt;
use image::io::Reader as ImageReader;
use mongodb::results::UpdateResult;
use crate::models::recipe_model::Image;

#[derive(FromForm)]
pub struct UploadImage<'v> {
    pub title: String,
    pub file: TempFile<'v>,
}

#[post("/image/<path>",  data = "<form>")]
pub async fn post_recipe_image(
    db: &State<MongoRepo>,
    path: String,
    mut form: Form<UploadImage<'_>>
) -> Result<Json<UpdateResult>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    match db.get_recipe(&id) {
        Ok(Some(mut recipe)) => {
            let some_path = std::env::temp_dir().join(form.file.name().unwrap());
            form.file.persist_to(&some_path).await.unwrap();
            let (width, height, _buf) = read_image(&some_path).await;

            recipe.images.push(Image{
                path: some_path.to_str().unwrap().to_string(),
                width,
                height,
                title: form.file.name().unwrap().parse().unwrap()
            });

            println!("width: {}, height: {}", width, height);
            let recipe_detail = db.update_recipe(&id, recipe);
            match recipe_detail {
                Ok(recipe) => Ok(Json(recipe)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        _ => Err(Status::BadRequest)
    }
}

async fn read_image(filename: &PathBuf) -> (i32, i32, Vec<u8>) {
    let mut fh = rocket::tokio::fs::File::open(filename).await.unwrap();
    let mut buf = Vec::new();
    fh.read_to_end(&mut buf).await.unwrap();
    let image = ImageReader::new(std::io::Cursor::new(&buf)).with_guessed_format().unwrap().decode().unwrap();
    let width = i32::try_from(image.width()).ok().unwrap();
    let height = i32::try_from(image.height()).ok().unwrap();
    drop(fh);
    return (width, height, buf);
}

#[post("/recipe", data = "<new_recipe>")]
pub async fn post_recipe(
    db: &State<MongoRepo>,
    new_recipe: Json<Recipe>
) -> Result<Json<InsertOneResult>, Status> {
    let recipe_detail = db.create_recipe(new_recipe.into_inner());
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipe/<path>")]
pub fn get_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<Recipe>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let recipe_detail = db.get_recipe(&id);

    match recipe_detail {
        Ok(Some(recipe)) => Ok(Json(recipe)),
        _ => Err(Status::InternalServerError),
    }
}

#[put("/recipe/<path>", data = "<new_recipe>")]
pub fn update_recipe(
    db: &State<MongoRepo>,
    path: String,
    new_recipe: Json<Recipe>,
) -> Result<Json<Recipe>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let mut data = new_recipe.into_inner();
    data.id = Some(id.clone());

    let update_result = db.update_recipe(&id, data);

    match update_result {
        Ok(update) => {
            return if update.matched_count == 1 {
                let updated_recipe_info = db.get_recipe(&id);

                match updated_recipe_info {
                    Ok(Some(recipe)) => Ok(Json(recipe)),
                    _ => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/recipe/<path>")]
pub fn delete_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let result = db.delete_recipe(&id);

    match result {
        Ok(res) => {
            return if res.deleted_count == 1 {
                Ok(Json("recipe successfully deleted!"))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => Err(Status::InternalServerError),
    }
}

#[get("/recipes")]
pub fn get_all_recipes(db: &State<MongoRepo>) -> Result<Json<Vec<Recipe>>, Status> {
    let recipes = db.get_all_recipes();

    match recipes {
        Ok(recipes) => Ok(Json(recipes)),
        Err(_) => Err(Status::InternalServerError),
    }
}
