mod shortest_path;
mod graph;

use std::time::Instant;
use rocket::serde::json::{json, Value};
use graph::Graph;

#[macro_use] 
extern crate rocket;

#[get("/<start>/<end>")]
fn search(start: String, end: String, graph: &rocket::State<Graph>) -> Value {
    let t = Instant::now();
    let r = shortest_path::find_shortest_path(start, end, graph.inner());
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
    
    let graph = Graph::load([
        "./databases/name_by_id",
        "./databases/id_by_name",
        "./databases/ingoing_edges_by_id",
        "./databases/outgoing_edges_by_id",
    ]);

    rocket::build()
        .manage(graph)
        .mount("/", routes![search])
        .register("/", catchers![not_found])

}