mod contributions;
pub mod cors;
mod dto;
pub mod health;
mod projects;

pub use contributions::*;
pub use projects::*;
use rocket_okapi::swagger_ui::SwaggerUIConfig;

pub use dto::*;
pub(crate) fn get_docs() -> SwaggerUIConfig {
	SwaggerUIConfig {
		url: "/openapi.json".to_string(),
		..Default::default()
	}
}
