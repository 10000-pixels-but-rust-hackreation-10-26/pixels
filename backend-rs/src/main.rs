use rocket::http::Method;
use rocket::{get, launch, routes};

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use rocket_ws as ws;
#[get("/ws")]
fn do_ws(ws: ws::WebSocket) -> ws::Stream!['static] {
let ws = ws.config(ws::Config {
        max_send_queue: Some(5),
        ..Default::default()
    });

    ws::Stream! { ws =>
        yield ws::Message::Text("{type : \"initial\"}".to_string());
        for await message in ws {
            println!("{:?}", message);
            yield message?;
        }
    }
}

#[get("/pixels")]
fn hello() -> String {
    let mut result = String::new();
    for _ in 0..10000 {
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

    rocket::build().attach(cors).mount("/", routes![hello, do_ws])
}
