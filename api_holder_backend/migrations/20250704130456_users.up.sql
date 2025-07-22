-- Add up migration script here

-- 1. Create the function
CREATE OR REPLACE FUNCTION update_updated_on_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_on = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE users
(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id CHAR(20) UNIQUE,
    first_name VARCHAR,
    last_name VARCHAR,
    email VARCHAR UNIQUE,
    phone CHAR(13) UNIQUE,
    Password VARCHAR,
    email_verified BOOL DEFAULT FALSE,
    phone_verified BOOL DEFAULT FALSE,
    created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TRIGGER set_updated_on_users
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_on_column();
