-- Your SQL goes here
-- create table users
CREATE TABLE users
(
    id UUID primary key default gen_random_uuid(),
    first_name VARCHAR,
    last_name VARCHAR,
    password VARCHAR,
    phone CHAR(13),
    email VARCHAR,
    created_on TIMESTAMPTZ DEFAULT NOW(),
    updated_on TIMESTAMPTZ DEFAULT NOW(),
    is_active BOOLEAN default TRUE
);

-- create a function update_updated_on_column
-- it can be used for every tables
CREATE OR REPLACE FUNCTION update_updated_on_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_on = NOW();
RETURN NEW;
END;
$$ language 'plpgsql';


-- create a trigger for updating the updated on column by update_updated_on_column
CREATE TRIGGER set_updated_on
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_on_column();