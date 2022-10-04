use std::path::PathBuf;
use image::io::Reader;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::fs::TempFile;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncReadExt;

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

#[derive(Debug,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub title: String
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image{
            id: self.id.to_owned(),
            path: self.path.to_owned(),
            width: self.width.to_owned(),
            height: self.height.to_owned(),
            title: self.title.to_owned()
        }
    }
}