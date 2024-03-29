use mongodb::bson::doc;
use mongodb::Database;
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use std::path::PathBuf;
use mongodb::bson::DateTime;
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::{Data, Request, Response, response, State};
use rocket::fs::TempFile;
use schemars::JsonSchema;

use crate::models::response::MessageResponse;
use crate::request_guards::basic::ApiKey;
use crate::db::{parse_id, image, recipe};
use crate::db::error::DbError;
use crate::errors::response::MyError;
use crate::models::image::{Image, ImageFile};
use crate::models::recipe::Recipe;

use uuid::Uuid;
use crate::routes::gifs::FileResponse;

#[derive(FromForm)]
pub struct ImageForm<'v> {
    pub title: String,
    pub file: TempFile<'v>,
}


#[post("/image/<id>",  data="<form>")]
pub async fn post_image(
    db: &State<Database>,
    id: String,
    mut form: Form<ImageForm<'_>>,
    _key: ApiKey,
) -> Result<Json<Recipe>, MyError> {
    let id = parse_id(&id)
            .map_err(|err|
                MyError::build(
                    Status::BadRequest.code,
                    Some(err.details)
                ))?;

    match recipe::find_one_recipe(&db, id).await {
        Ok(Some(mut recipe)) => {
            let temp_path = std::env::temp_dir().join(form.title.as_str());
            form.file.persist_to(&temp_path).await.unwrap();
            let image_file = ImageFile::read(&temp_path).await;
            let mut image = Image {
                _id: "".to_string(),
                path: temp_path.to_str().unwrap().to_string(),
                width: image_file.width,
                height: image_file.height,
                title: form.file.name().unwrap().parse().unwrap(),
                created_at: DateTime::now().to_string()
            };

            let image_id = image::insert_image(&db,image.clone())
                .await
                .map(|result| result.inserted_id.as_object_id())
                .map_err(|_err| {
                    MyError::build(
                        Status::BadRequest.code,
                        Some("Invalid input.".to_string())
                    )
                })?
                .ok_or({
                    MyError::build(
                        Status::BadRequest.code,
                        Some("No Object ID found!".to_string())
                    )
                })?;

            image._id = image_id.to_string();

            recipe.images.push(image.clone());

            match recipe::update_recipe(&db, id, recipe).await {
                Ok(Some(mut recipe)) => {
                    recipe.images.push(image);
                    Ok(Json(recipe))
                },
                _ => Err(MyError::build(
                    Status::InternalServerError.code,
                    Some("Updating recipe with new image failed.".to_string())
                )),
            }
        }
        _ => {
            Err(MyError::build(
                Status::NotFound.code,
                Some("Could not find the image.".to_string())
            ))
        }
    }
}

#[openapi(tag = "Image")]
#[get("/image/<id>")]
pub async fn get_image(
    db: &State<Database>,
    id: String,
    _key: ApiKey,
) -> Result<FileResponse, MyError> {
    let id =
        parse_id(&id).map_err(|err| MyError::build(Status::BadRequest.code, Some(err.details)))?;

    match image::find_one_image(&db, id).await {
        Ok(image) => match image {
            Some(image) => {
                let mut path = PathBuf::new();
                path.push(image.path);
                let image_file = ImageFile::read(&path).await;
                Ok(FileResponse((ContentType::JPEG, image_file.data)))
            },
            None => Err(MyError::build(
                Status::NotFound.code,
                Some("Could not find the image.".to_string())
            ))
        },
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(
                Status::BadRequest.code,
                Some(format!("Image not found with _id {}", &id)),
            ));
        }
    }
}

#[openapi(tag = "Image")]
#[delete("/image/<id>")]
pub async fn delete_image(
    db: &State<Database>,
    id: String,
    _key: ApiKey,
) -> Result<Json<&str>, MyError> {
    let id =
        parse_id(&id).map_err(|err| MyError::build(Status::BadRequest.code, Some(err.details)))?;
    return match image::delete_one_image(&db, id).await {
        Ok(res) => {
            if res.deleted_count == 1 {
                Ok(Json("Image successfully deleted!"))
            } else {
                Err(MyError::build(
                    Status::NotFound.code,
                    Some("Not Found.".to_string()),
                ))
            }
        }
        Err(error) => {
            println!("{:?}", error);
            Err(MyError::build(
                Status::BadRequest.code,
                Some(format!("Image not found with _id {}", &id)),
            ))
        }
    };
}

#[openapi(tag = "Image")]
#[get("/images")]
pub async fn get_all_images(
    db: &State<Database>,
    _key: ApiKey,
) -> Result<Json<Vec<Image>>, Status> {
    match image::find_all_images(&db).await {
        Ok(images) => Ok(Json(images)),
        _ => Err(Status::InternalServerError),
    }
}
