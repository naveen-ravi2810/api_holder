-- This file should undo anything in `up.sql`
-- 1. Drop the trigger from the users table
DROP TRIGGER IF EXISTS set_updated_on ON users;

-- 2. Drop the trigger function
DROP FUNCTION IF EXISTS update_updated_on_column();

-- 3. Drop the users table
DROP TABLE IF EXISTS users;