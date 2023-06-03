use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

	HttpServer::new(|| {
		App::new()
			.app_data(web::Data::new(dictionary::AppState {
				db_connection: dictionary::db::establish_connection(),
			}))
			.configure(dictionary::config)
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
