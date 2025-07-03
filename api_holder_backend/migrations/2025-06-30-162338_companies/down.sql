-- This file should undo anything in `up.sql`
DROP TRIGGER if EXISTS set_updated_on on companies;
DROP TABLE companies;
DROP TYPE if EXISTS COMPANY_TYPE;
