pub mod customer;
pub mod response;
pub mod recipe;
pub mod image;
pub mod gif;


pub trait DocumentConvertable<T> {
    fn to_document(&self) -> T;
}

pub trait ObjectConvertable<T> {
    fn to_object(&self) -> T;
}