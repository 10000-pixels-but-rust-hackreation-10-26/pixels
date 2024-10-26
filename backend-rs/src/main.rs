use rocket::futures::SinkExt;
use rocket::http::Method;
use rocket::{get, launch, routes, State};

use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_ws as ws;

mod pixel_store;
mod sockets;

#[get("/ws")]
fn do_ws(ws: ws::WebSocket) -> ws::Channel<'static> {
    let ws = ws.config(ws::Config {
        max_send_queue: Some(5),
        ..Default::default()
    });

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let message = serde_json::to_string(&sockets::Initial {
                r#type: "initial".to_string(),
            })
            .expect("aaa");
            let _ = stream.send(message.into()).await;
            Ok(())
        })
    })
}

#[get("/pixels")]
fn pixels(pixel_store: &State<Box<pixel_store::PixelStore>>) -> String {
    let result = pixel_store
        .data
        .map(|x| (x as u8 + b'0'))
        .into_iter()
        .collect::<Vec<u8>>();
    String::from_utf8(result).unwrap()
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
        .mount("/", routes![pixels, do_ws])
        .manage(cs)
}
