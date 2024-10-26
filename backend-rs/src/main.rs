use rocket::http::Method;
use rocket::{get, launch, routes};

use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[get("/pixels")]
fn hello() -> String {
    let mut result = String::new();
    for x in 0..10000 {
        result += "5"
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
    .to_cors().unwrap();

    rocket::build().attach(cors).mount("/", routes![hello])
}
