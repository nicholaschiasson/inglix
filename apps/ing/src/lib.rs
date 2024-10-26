use askama::Template;
use axum::{response::Html, routing::get, Router};
use pulldown_cmark::{Options, Parser};
use tower_http::services::{ServeDir, ServeFile};

pub trait AppState: Clone + Send + Sync + 'static {}

pub fn router<S: AppState>() -> Router<S> {
	Router::new()
		.route("/wiki", get(get_wiki::<S>))
		.nest_service(
			"/",
			ServeDir::new("rsrc").not_found_service(ServeFile::new("rsrc/notfound.html")),
		)
}

#[derive(Template)]
#[template(path = "wiki.html", escape = "none")]
struct WikiTemplate {
	wiki: String,
}

async fn get_wiki<S: AppState>() -> Result<Html<String>, Html<String>> {
	let readme_md = include_str!("../../../README.md");

	let parser = Parser::new_ext(readme_md, Options::all());

	let mut readme_html = String::new();
	pulldown_cmark::html::push_html(&mut readme_html, parser);

	Ok(Html(
		WikiTemplate { wiki: readme_html }
			.render()
			.map_err(|e| e.to_string())?,
	))
}
