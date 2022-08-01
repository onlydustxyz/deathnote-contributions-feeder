mod contributions;
pub mod cors;
pub mod health;
mod projects;
mod utils;

pub use contributions::*;
pub use projects::*;
use rocket_okapi::swagger_ui::SwaggerUIConfig;
pub use utils::{api_key, hex_prefixed_string};

pub use utils::*;
pub(crate) fn get_docs() -> SwaggerUIConfig {
	SwaggerUIConfig {
		url: "/openapi.json".to_string(),
		..Default::default()
	}
}

pub mod contributors;
