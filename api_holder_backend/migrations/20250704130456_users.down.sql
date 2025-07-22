-- Add down migration script here
DROP TRIGGER if exists set_updated_on_users on users ;
DROP TABLE users;
DROP FUNCTION if exists update_updated_on_column;