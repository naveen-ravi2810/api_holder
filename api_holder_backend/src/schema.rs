// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "company_type"))]
    pub struct CompanyType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CompanyType;

    companies (id) {
        id -> Uuid,
        company_name -> Nullable<Varchar>,
        company_type -> Nullable<CompanyType>,
        created_by_user -> Nullable<Uuid>,
        created_on -> Nullable<Timestamptz>,
        updated_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        #[max_length = 13]
        phone -> Nullable<Bpchar>,
        email -> Nullable<Varchar>,
        created_on -> Nullable<Timestamptz>,
        updated_on -> Nullable<Timestamptz>,
        is_active -> Nullable<Bool>,
    }
}

diesel::joinable!(companies -> users (created_by_user));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    users,
);
