use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Machine {
    name: String,
    count: i32,
}

#[get("/machines")]
async fn get_machines() -> impl Responder {
    let db = Connection::open("../db.sqlite").unwrap();
    let mut stmt = db.prepare("SELECT name, count FROM machines").unwrap();
    let machines = stmt.query_map([], |row| {
        Ok(Machine {
            name: row.get(0)?,
            count: row.get(1)?,
        })
    }).unwrap().map(|machine| machine.unwrap()).collect::<Vec<Machine>>();
    // json response
    HttpResponse::Ok().json(machines)
}

#[post("/machines")]
async fn add_machine(machine: web::Json<Machine>) -> impl Responder {
    let db = Connection::open("../db.sqlite").unwrap();
    db.execute("INSERT INTO machines (name, count) VALUES (?1, ?2)", params![&machine.name, &machine.count]).unwrap();
    HttpResponse::Ok()
}


#[put("/machines")]
async fn update_machine(machine: web::Json<Machine>) -> impl Responder {
    let db = Connection::open("../db.sqlite").unwrap();
    db.execute("UPDATE machines SET count = ?1 WHERE name = ?2", params![&machine.count, &machine.name]).unwrap();
    HttpResponse::Ok()
}

#[delete("/machines")]
async fn delete_machine(machine: web::Json<Machine>) -> impl Responder {
    let db = Connection::open("../db.sqlite").unwrap();
    db.execute("DELETE FROM machines WHERE name = ?1", params![&machine.name]).unwrap();
    HttpResponse::Ok()
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //init the database
    let db = Connection::open("../db.sqlite").unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS machines (name TEXT PRIMARY KEY, count INTEGER)", []).unwrap();
    //start the server
    HttpServer::new(|| {
        App::new()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}