use std::path::PathBuf;
use mongodb::bson::oid::ObjectId;
use rocket::form::Form;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{Request, Response, response, State};
use crate::{MongoRepo};
use crate::models::image_model::{Image, ImageFile, UploadImage};
use crate::models::recipe_model::Recipe;

pub struct ImageResponse(pub (ContentType, Vec<u8>));

impl<'r> Responder<'r, 'static> for ImageResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let csp = "default-src 'self';";
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Content-Security-Policy", csp)
            .ok()
    }
}

#[post("/image/<id>",  data = "<form>")]
pub async fn create_image(
    db: &State<MongoRepo>,
    id: String,
    mut form: Form<UploadImage<'_>>
) -> Result<Json<Recipe>, Status> {
    match db.get_recipe(&id).await {
        Ok(Some(mut recipe)) => {
            let temp_path = std::env::temp_dir().join(form.file.name().unwrap());
            form.file.persist_to(&temp_path).await.unwrap();
            let image_file = ImageFile::read(&temp_path).await;
            let mut image = Image {
                id: None,
                path: temp_path.to_str().unwrap().to_string(),
                width: image_file.width,
                height: image_file.height,
                title: form.file.name().unwrap().parse().unwrap()
            };

            let image_id = db.create_image(&image)
                .await
                .map(|result| result.inserted_id.as_object_id())
                .map_err(|_err| {
                    error!("{:?}", _err);
                    Status::InternalServerError
                })?
                .ok_or({
                    error!("No Object ID found!");
                    Status::InternalServerError
                })?;

            image.id = Some(image_id);

            recipe.images.push(image.clone());

            match db.update_recipe(&id, recipe).await {
                Ok(Some(mut recipe)) => {
                    recipe.images.push(image);
                    Ok(Json(recipe))
                },
                _ => Err(Status::InternalServerError),
            }
        }
        _ => Err(Status::BadRequest)
    }
}

#[get("/image/<id>")]
pub async fn get_image(db: &State<MongoRepo>, id: String) -> Result<ImageResponse, Status> {
    let image = db.get_image(&id)
        .await
        .map_err(|_err| Status::InternalServerError)?;

    match image {
        None => Err(Status::InternalServerError),
        Some(image) => {
            let mut path = PathBuf::new();
            path.push(image.path);
            let image_file = ImageFile::read(&path).await;
            Ok(ImageResponse((ContentType::JPEG, image_file.data)))
        }
    }
}

#[delete("/image/<id>")]
pub async fn delete_image(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    match db.delete_image(&id).await {
        Ok(res) => {
            return if res.deleted_count == 1 {
                Ok(Json("image successfully deleted!"))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_err) => Err(Status::InternalServerError),
    }
}