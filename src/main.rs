mod shortest_path;

use std::time::Instant;
use rocket::serde::json::{json, Value};

#[macro_use] 
extern crate rocket;

#[get("/<start>/<end>")]
fn search(start: String, end: String) -> Value {
    let t = Instant::now();
    let r = shortest_path::find_shortest_path(start, end);
    match r {
        Ok(path) => {
            return json!({
                "path": path,
                "time": t.elapsed().as_millis()
            })
        },
        Err(e) => {
            return json!({
                "error": e,
                "time": t.elapsed().as_millis()
            })
        }
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![search])
        .register("/", catchers![not_found])
}