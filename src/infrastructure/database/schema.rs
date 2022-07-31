table! {
	applications (id) {
		id -> Uuid,
		contribution_id -> Uuid,
		contributor_id -> Varchar,
	}
}

table! {
	contributions (id) {
		onchain_id -> Varchar,
		project_id -> Varchar,
		status -> Varchar,
		transaction_hash -> Nullable<Varchar>,
		contributor_id -> Varchar,
		gate -> Int2,
		title -> Nullable<Text>,
		description -> Nullable<Text>,
		external_link -> Nullable<Text>,
		difficulty -> Nullable<Text>,
		technology -> Nullable<Text>,
		duration -> Nullable<Text>,
		context -> Nullable<Text>,
		#[sql_name = "type"]
		type_ -> Nullable<Text>,
		validator -> Varchar,
		id -> Uuid,
	}
}

table! {
	projects (id) {
		id -> Varchar,
		owner -> Varchar,
		name -> Varchar,
		last_indexed_time -> Nullable<Timestamp>,
	}
}

joinable!(applications -> contributions (contribution_id));
joinable!(contributions -> projects (project_id));

allow_tables_to_appear_in_same_query!(applications, contributions, projects,);
