-- Your SQL goes here
CREATE TYPE COMPANY_TYPE AS ENUM
(
    'start_up', --Small, fast-growing companies; tech-driven, may use many SaaS tools.
    'enterprise', --Large organizations with multiple departments; often need custom solutions.
    'SMB', --Small or medium-sized businesses; need scalable, easy-to-use tools.
    'non-Profit', --Organizations focused on charity, education, etc.; cost-sensitive.
    'government' --Public sector, regulated, may need security/compliance-based solutions.
);

CREATE TABLE companies
(
    id uuid primary key default gen_random_uuid(),
    company_name VARCHAR,
    company_type COMPANY_TYPE,
    created_by_user uuid,
    created_on TIMESTAMPTZ DEFAULT NOW(),
    updated_on TIMESTAMPTZ DEFAULT NOW(),
    FOREIGN KEY (created_by_user) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TRIGGER set_updated_on
BEFORE UPDATE ON companies
FOR EACH ROW
EXECUTE FUNCTION update_updated_on_column();