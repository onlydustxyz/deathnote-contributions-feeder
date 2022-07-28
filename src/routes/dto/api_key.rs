use okapi::openapi3::{Object, Parameter};
use rocket::{
	http::Status,
	outcome::Outcome,
	request::{self, FromRequest},
	Request,
};
use rocket_okapi::{
	gen::OpenApiGenerator,
	request::{OpenApiFromRequest, RequestHeaderInput},
};
pub struct ApiKey(String);

fn is_valid_key(key: &str) -> bool {
	key == match std::env::var("API_KEY") {
		Ok(v) => v,
		Err(_) => return false,
	}
}

#[derive(Debug)]
pub enum ApiKeyError {
	BadCount,
	Missing,
	Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
	type Error = ApiKeyError;

	async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		let keys: Vec<_> = request.headers().get("Api-Key").collect();
		match keys.len() {
			0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
			1 if is_valid_key(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
			1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
			_ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
		}
	}
}

impl<'r> OpenApiFromRequest<'r> for ApiKey {
	fn from_request_input(
		_gen: &mut OpenApiGenerator,
		_name: String,
		_required: bool,
	) -> rocket_okapi::Result<RequestHeaderInput> {
		Ok(RequestHeaderInput::Parameter(Parameter {
			name: "Api-Key".to_string(),
			location: "header".to_string(),
			description: None,
			required: true,
			deprecated: false,
			allow_empty_value: false,
			value: okapi::openapi3::ParameterValue::Schema {
				style: None,
				explode: None,
				allow_reserved: true,
				schema: okapi::openapi3::SchemaObject::default(),
				example: None,
				examples: None,
			},
			extensions: Object::default(),
		}))
	}

	fn get_responses(
		_gen: &mut OpenApiGenerator,
	) -> rocket_okapi::Result<okapi::openapi3::Responses> {
		Ok(okapi::openapi3::Responses::default())
	}
}
