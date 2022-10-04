use std::path::PathBuf;
use crate::{models::recipe_model::Recipe, repository::mongodb_repo::MongoRepo, serde_json};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, Request, response, Response, serde::json::Json, State};
use rocket::form::Form;
use rocket::fs::TempFile;
use tokio::io::AsyncReadExt;
use image::io::Reader as ImageReader;
use rocket::http::ContentType;

use rocket::response::Responder;

use crate::models::recipe_model::Image;

#[derive(FromForm)]
pub struct UploadImage<'v> {
    pub title: String,
    pub file: TempFile<'v>,
}

pub struct ImageFile {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}


pub async fn read_image(filename: &PathBuf) -> ImageFile {
    let mut fh = rocket::tokio::fs::File::open(filename).await.unwrap();
    let mut data = Vec::new();
    fh.read_to_end(&mut data).await.unwrap();
    let image = ImageReader::new(std::io::Cursor::new(&data)).with_guessed_format().unwrap().decode().unwrap();
    let width = i32::try_from(image.width()).ok().unwrap();
    let height = i32::try_from(image.height()).ok().unwrap();
    drop(fh);
    ImageFile {
        width,
        height,
        data
    }
}

#[post("/recipe", data = "<new_recipe>")]
pub async fn post_recipe(
    db: &State<MongoRepo>,
    new_recipe: Json<Recipe>
) -> Result<Json<InsertOneResult>, Status> {
    let recipe_detail = db.create_recipe(new_recipe.into_inner()).await;
    match recipe_detail {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/recipe/<id>")]
pub async fn get_recipe(db: &State<MongoRepo>, id: String) -> Result<Json<Recipe>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let recipe = db.get_recipe(&id).await;

    match recipe {
        Ok(Some(recipe)) => Ok(Json(recipe)),
        _ => Err(Status::InternalServerError),
    }
}

#[put("/recipe/<path>", data = "<new_recipe>")]
pub async fn update_recipe(
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
    data.id = None;

    let update_result = db.update_recipe(&id, data).await;

    match update_result {
        Ok(Some(recipes)) => Ok(Json(recipes)),
        _ => Err(Status::InternalServerError),
    }
}

#[delete("/recipe/<path>")]
pub async fn delete_recipe(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let result = db.delete_recipe(&id).await;

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
pub async fn get_all_recipes(db: &State<MongoRepo>) -> Result<Json<Vec<Recipe>>, Status> {
    let recipes = db.get_all_recipes().await;

    match recipes {
        Ok(recipes) => Ok(Json(recipes)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[post("/image/<id>",  data = "<form>")]
pub async fn post_recipe_image(
    db: &State<MongoRepo>,
    id: String,
    mut form: Form<UploadImage<'_>>
) -> Result<Json<Recipe>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    match db.get_recipe(&id).await {
        Ok(Some(mut recipe)) => {
            let some_path = std::env::temp_dir().join(form.file.name().unwrap());
            form.file.persist_to(&some_path).await.unwrap();
            let image_file = read_image(&some_path).await;
            let image = Image {
                path: some_path.to_str().unwrap().to_string(),
                width: image_file.width,
                height: image_file.height,
                title: form.file.name().unwrap().parse().unwrap()
            };

            let insert_result = db.create_image(&image)
                .await
                .map_err(|err| Status::InternalServerError)?;

            recipe.images.push(image);

            let recipe_detail = db.update_recipe(&id, recipe).await;
            match recipe_detail {
                Ok(Some(recipe)) => Ok(Json(recipe)),
                _ => Err(Status::InternalServerError),
            }
        }
        _ => Err(Status::BadRequest)
    }
}

pub struct Img((ContentType, Vec<u8>));

impl<'r> Responder<'r, 'static> for Img {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let csp = "default-src 'self';";
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Content-Security-Policy", csp)
            .ok()
    }
}

#[get("/image/<id>")]
pub async fn get_image(db: &State<MongoRepo>, id: String) -> Result<Img, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let id = ObjectId::parse_str(id).unwrap();

    let image = db.get_image(&id)
        .await
        .map_err(|err| Status::InternalServerError)?;

    let mut path = PathBuf::new();

    match image {
        None => Err(Status::InternalServerError),
        Some(image) => {
            path.push(image.path);
            let image_file = read_image(&path).await;
            Ok(Img((ContentType::JPEG, image_file.data)))
        }
    }
}