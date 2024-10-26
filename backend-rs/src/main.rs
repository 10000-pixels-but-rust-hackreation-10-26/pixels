use rocket::http::Method;
use rocket::{get, launch, routes, State};

use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod pixel_store;

#[get("/pixels")]
fn pixels(pixel_store: &State<Box<pixel_store::PixelStore>>) -> String {
    let mut result = String::new();
    for x in pixel_store.data {
        result += (x as u8).to_string().as_str() //There's no way this is performant
    }
    result
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let cs = Box::new(pixel_store::PixelStore::new());

    rocket::build()
        .attach(cors)
        .mount("/", routes![pixels])
        .manage(cs)
}
