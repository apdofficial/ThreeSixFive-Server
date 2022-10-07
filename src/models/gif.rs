use mongodb::bson::DateTime;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeStepDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub description: String,
    pub gif: Gif,
    pub created_at: DateTime,
}

impl RecipeStepDocument{
    pub(crate) fn to_object(&self) -> RecipeStep{
        RecipeStep{
            _id: self._id.clone().unwrap_or(ObjectId::new()).to_string(),
            description: "".to_string(),
            gif: Gif {
                path: self.gif.path.clone(),
                width: self.gif.width.clone(),
                height: self.gif.height.clone(),
                title: self.gif.title.clone(),
            },
            created_at: self.created_at.to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct RecipeStep {
    pub _id: String,
    pub description: String,
    pub gif: Gif,
    pub created_at: String,
}

impl RecipeStep {
    pub(crate) fn to_document(&self) -> RecipeStepDocument{
        RecipeStepDocument {
            _id: None,
            description: self.description.clone(),
            gif: Gif {
                path: self.gif.path.clone(),
                width: self.gif.width.clone(),
                height: self.gif.height.clone(),
                title: self.gif.title.clone()
            },
            created_at: DateTime::now()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Gif {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub title: String
}
