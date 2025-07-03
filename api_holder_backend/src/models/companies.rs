use diesel_derive_enum::DbEnum;
use crate::sql_types::CompanyType;

#[derive(Debug, DbEnum, PartialEq)]
#[DieselTypePath = "crate::sql_types::CompanyType"]
#[pg_type = "company_type"]
pub enum CompanyKind {
    #[db_rename = "start_up"]
    StartUp,
    #[db_rename = "enterprise"]
    Enterprise,
    #[db_rename = "SMB"]
    Smb,
    #[db_rename = "non-Profit"]
    NonProfit,
    #[db_rename = "government"]
    Government,
}
