use rocket::http::Method;
use rocket::{get, launch, routes, State};

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

mod pixel_store;


#[get("/pixels")]
fn pixels(pixel_store: &State<Box<pixel_store::PixelStore>>) -> String {
    let mut result = String::new();
<<<<<<< HEAD
    for _ in 0..10000 {
        result += "5"
=======
    for x in pixel_store.data {
        result += (x as u8).to_string().as_str() //There's no way this is performant
>>>>>>> f8c1dd0f62a35a6a5e94288a7e411d08da5040de
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

<<<<<<< HEAD
    rocket::build().attach(cors).mount("/", routes![hello, do_ws])
=======
    let cs = Box::new(pixel_store::PixelStore::new());

    rocket::build()
        .attach(cors)
        .mount("/", routes![pixels])
        .manage(cs)
>>>>>>> f8c1dd0f62a35a6a5e94288a7e411d08da5040de
}
