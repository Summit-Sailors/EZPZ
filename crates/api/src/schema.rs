// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "e_db_experience_level"))]
    pub struct EDbExperienceLevel;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "e_db_internal_status"))]
    pub struct EDbInternalStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "e_db_pay_type"))]
    pub struct EDbPayType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "e_db_search_query"))]
    pub struct EDbSearchQuery;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "e_db_time_estimate"))]
    pub struct EDbTimeEstimate;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EDbSearchQuery;
    use super::sql_types::EDbInternalStatus;
    use super::sql_types::EDbExperienceLevel;
    use super::sql_types::EDbTimeEstimate;
    use super::sql_types::EDbPayType;

    postings (id) {
        id -> Uuid,
        uid -> Int8,
        url -> Text,
        search_query -> EDbSearchQuery,
        internal_status -> EDbInternalStatus,
        title -> Text,
        description -> Text,
        skills -> Array<Text>,
        experience_level -> EDbExperienceLevel,
        time_estimate -> Nullable<EDbTimeEstimate>,
        pay_type -> EDbPayType,
        budget -> Nullable<Int4>,
        max_rate -> Nullable<Int4>,
        min_rate -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tool_results (id) {
        id -> Uuid,
        term -> Text,
        stage -> Text,
        tech_field -> Text,
        sectors -> Array<Text>,
        conditions -> Array<Text>,
        miscs -> Array<Text>,
        languages -> Array<Text>,
        local_project -> Text,
        global_project -> Text,
        frontier_project -> Text,
        targeted_pitch -> Text,
    }
}

diesel::joinable!(tool_results -> postings (id));

diesel::allow_tables_to_appear_in_same_query!(
    postings,
    tool_results,
);
