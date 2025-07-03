use diesel::sql_types::SqlType;
use diesel_derive_enum::DbEnum;

#[derive(SqlType)]
#[postgres(type_name = "company_type")]
pub struct CompanyType;
