#[macro_use] extern crate rocket;

mod api;
mod models;
mod repository;
mod helpers;
mod tests;

use rocket::serde::json::serde_json;
use api::user_api::{create_user, delete_user, get_user, update_user, get_all_users};
use api::recipe_api::{
    create_recipe,
    delete_recipe,
    get_recipe,
    update_recipe,
    get_all_recipes,
};
use api::image_api::{
    create_image,
    get_image,
    delete_image
};
use repository::mongodb_repo::MongoRepo;
use helpers::mock;

#[launch]
fn rocket() -> _ {
    let dummy_recipe = mock::get_recipe();

    println!("{}", serde_json::to_string(&dummy_recipe).unwrap());

    let db = MongoRepo::init();
    rocket::build()
        .manage(db)

        .attach(fairings::cors::CORS)

        .mount(
            "/",
            openapi_get_routes![
                routes::index,
                routes::customer::get_customers,
                routes::customer::get_customer_by_id,
                routes::customer::post_customer,
                routes::customer::patch_customer_by_id,
                routes::customer::delete_customer_by_id
            ],
        )

        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])

        .mount("/", routes![create_recipe])
        .mount("/", routes![delete_recipe])
        .mount("/", routes![get_recipe])
        .mount("/", routes![update_recipe])
        .mount("/", routes![get_all_recipes])

        .mount("/", routes![create_image])
        .mount("/", routes![get_image])
        .mount("/", routes![delete_image])

        .mount(
            "/api-docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

// Unit testings
#[cfg(test)]
mod tests;