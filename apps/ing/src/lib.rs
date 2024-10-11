use askama::Template;
use axum::{extract::State, response::Html, routing::get, Router};
use pulldown_cmark::{Options, Parser};
use reqwest::Client;
use tower_http::services::{ServeDir, ServeFile};

pub trait AppState: Clone + Send + Sync + 'static {
	fn client(&self) -> &Client;
}

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

async fn get_wiki<S: AppState>(State(state): State<S>) -> Result<Html<String>, Html<String>> {
	let readme_md = state
		.client()
		.get("https://raw.githubusercontent.com/nicholaschiasson/inglix/refs/heads/main/README.md")
		.send()
		.await
		.map_err(|e| e.to_string())?
		.text()
		.await
		.map_err(|e| e.to_string())?;

	let mut parser_options = Options::empty();
	parser_options.insert(Options::ENABLE_FOOTNOTES);
	parser_options.insert(Options::ENABLE_GFM);
	parser_options.insert(Options::ENABLE_STRIKETHROUGH);
	parser_options.insert(Options::ENABLE_TABLES);

	let parser = Parser::new_ext(&readme_md, parser_options);

	let mut readme_html = String::new();
	pulldown_cmark::html::push_html(&mut readme_html, parser);

	println!("{readme_html}");

	Ok(Html(
		WikiTemplate { wiki: readme_html }
			.render()
			.map_err(|e| e.to_string())?,
	))
}
