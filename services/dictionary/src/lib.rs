#[macro_use]
extern crate diesel;

use actix_web::{web, Responder};
use diesel::PgConnection;
use uuid::Uuid;

// use crate::models::{NewWord, Word};

pub mod db;
mod models;
mod schema;

pub struct AppState {
	pub db_connection: PgConnection,
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/words")
			// .service(create_word)
			.service(read_word)
			.service(update_word)
			.service(delete_word),
	);
}

// #[actix_web::post("/")]
// async fn create_word(word: CreateWord, data: web::Data<AppState>) {
// }

#[actix_web::get("/{word_id}")]
async fn read_word(word_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
	"hola"
}

#[actix_web::put("/{word_id}")]
async fn update_word(word_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
	"nino"
}

#[actix_web::delete("/{word_id}")]
async fn delete_word(word_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
	"bambino"
}
