use std::path::PathBuf;
use mongodb::bson::DateTime;
use image::io::Reader;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::fs::TempFile;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncReadExt;
use schemars::JsonSchema;

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

impl ImageFile{
    pub async fn read(filename: &PathBuf) -> ImageFile {
        let mut fh = rocket::tokio::fs::File::open(filename).await.unwrap();
        let mut data = Vec::new();
        fh.read_to_end(&mut data).await.unwrap();
        let image = Reader::new(std::io::Cursor::new(&data)).with_guessed_format().unwrap().decode().unwrap();
        let width = i32::try_from(image.width()).ok().unwrap();
        let height = i32::try_from(image.height()).ok().unwrap();
        drop(fh);
        ImageFile { width, height, data }
    }
}

#[derive(Debug,Serialize, Deserialize, JsonSchema, Clone)]
pub struct Image {
    pub _id: String,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub created_at: String,
}

impl Image {
    pub(crate) fn to_document(&self) -> ImageDocument{
        ImageDocument{
            _id: None,
            path: self.path.clone(),
            width: self.width,
            height: self.height,
            title: self.title.clone(),
            created_at: DateTime::now()
        }
    }
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct ImageDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub created_at: DateTime,
}

impl ImageDocument {
    pub(crate) fn to_object(&self) -> Image{
        Image{
            _id: self._id.clone().unwrap_or(ObjectId::new()).to_string(),
            path: self.path.clone(),
            width: self.width,
            height: self.height,
            title: self.title.clone(),
            created_at: self.created_at.to_string()
        }
    }
}