use deathnote_contributions_feeder::domain::ContributorId;
use schemars::{
	gen::SchemaGenerator,
	schema::{InstanceType, Schema, SchemaObject},
	JsonSchema,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContributorIdDto(ContributorId);

impl JsonSchema for ContributorIdDto {
	fn schema_name() -> String {
		"ContributorId".to_string()
	}

	fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
		let schema = SchemaObject {
			instance_type: Some(InstanceType::String.into()),
			format: Some("hex".to_string()),
			..Default::default()
		};

		schema.into()
	}
}

impl From<ContributorIdDto> for ContributorId {
	fn from(wrapper: ContributorIdDto) -> Self {
		wrapper.0
	}
}
