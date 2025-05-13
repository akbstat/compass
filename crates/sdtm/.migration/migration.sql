-- define auto update field updated_at
CREATE OR REPLACE FUNCTION update_timestamp() RETURNS TRIGGER AS $update_timestamp$ BEGIN NEW.updated_at = now();
RETURN NEW;
END;
$update_timestamp$ LANGUAGE plpgsql;
-- define table lang
CREATE TABLE IF NOT EXISTS lang (
    id SERIAL PRIMARY KEY,
    name VARCHAR(10) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN lang.name IS 'Language Name';
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER lang_update BEFORE
UPDATE ON lang FOR EACH ROW EXECUTE FUNCTION update_timestamp();
--define table variable_type
CREATE TABLE IF NOT EXISTS variable_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(15) NOT NULL,
    lang_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_variable_type ON variable_type (name, lang_id);
COMMENT ON COLUMN variable_type.name IS 'Variable Type Name';
COMMENT ON COLUMN variable_type.lang_id IS 'Language ID';
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER variable_type_update BEFORE
UPDATE ON variable_type FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table variable_core
CREATE TABLE IF NOT EXISTS variable_core (
    id SERIAL PRIMARY KEY,
    name VARCHAR(15) NOT NULL,
    lang_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_variable_core ON variable_core (name, lang_id);
COMMENT ON COLUMN variable_core.name IS 'Variable Core Name';
COMMENT ON COLUMN variable_core.lang_id IS 'Language ID';
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER variable_core_update BEFORE
UPDATE ON variable_core FOR EACH ROW EXECUTE FUNCTION update_timestamp();
--define table variable_role
CREATE TABLE IF NOT EXISTS variable_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(15) NOT NULL,
    lang_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_variable_role ON variable_role (name, lang_id);
COMMENT ON COLUMN variable_role.name IS 'Variable Role Name';
COMMENT ON COLUMN variable_role.lang_id IS 'Language ID';
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER variable_role_update BEFORE
UPDATE ON variable_role FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table sdtm_version
CREATE TABLE IF NOT EXISTS sdtm_version (
    id SERIAL PRIMARY KEY,
    name VARCHAR(10) NOT NULL,
    lang_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN sdtm_version.name IS 'SDTMIG Version Name';
COMMENT ON COLUMN sdtm_version.lang_id IS 'Language ID';
CREATE UNIQUE INDEX IF NOT EXISTS idx_sdtm_version ON sdtm_version (name, lang_id);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER sdtm_version_update BEFORE
UPDATE ON sdtm_version FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table sdtm_domain
CREATE TABLE IF NOT EXISTS sdtm_domain (
    id SERIAL PRIMARY KEY,
    name VARCHAR(10) NOT NULL,
    description VARCHAR(200) NOT NULL,
    version_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN sdtm_domain.name IS 'Domain Name in SDTM';
COMMENT ON COLUMN sdtm_domain.description IS 'Domain Description';
COMMENT ON COLUMN sdtm_domain.version_id IS 'Version ID';
CREATE UNIQUE INDEX IF NOT EXISTS idx_sdtm_domain ON sdtm_domain (name, version_id);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER sdtm_domain_update BEFORE
UPDATE ON sdtm_domain FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table sdtm_variable
CREATE TABLE IF NOT EXISTS sdtm_variable (
    id SERIAL PRIMARY KEY,
    name VARCHAR(10) NOT NULL,
    label VARCHAR(200) NOT NULL,
    domain_id INTEGER NOT NULL,
    variable_type_id INTEGER NOT NULL,
    codelist VARCHAR(100),
    variable_core_id INTEGER NOT NULL,
    variable_role_id INTEGER NOT NULL,
    variable_order INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN sdtm_variable.name IS 'Variable Name in Domain';
COMMENT ON COLUMN sdtm_variable.label IS 'Variable Label';
COMMENT ON COLUMN sdtm_variable.domain_id IS 'Domain ID';
COMMENT ON COLUMN sdtm_variable.variable_type_id IS 'Variable Type ID';
COMMENT ON COLUMN sdtm_variable.codelist IS 'CodeList';
COMMENT ON COLUMN sdtm_variable.variable_core_id IS 'Variable Core ID';
COMMENT ON COLUMN sdtm_variable.variable_role_id IS 'Variable Role ID';
COMMENT ON COLUMN sdtm_variable.variable_order IS 'Variable Order in Domain';
CREATE UNIQUE INDEX IF NOT EXISTS idx_sdtm_variable ON sdtm_variable (name, domain_id);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER sdtm_variable_update BEFORE
UPDATE ON sdtm_variable FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- create view for sdtm variable detail
CREATE OR REPLACE VIEW sdtm_variable_view AS
SELECT sdtm_variable.domain_id,
    sdtm_variable.id AS variable_id,
    sdtm_variable.name AS variable_name,
    sdtm_variable.label AS variable_label,
    variable_type.name AS variable_type,
    sdtm_variable.codelist,
    variable_core.name AS variable_core,
    variable_role.name AS variable_role,
    sdtm_variable.variable_order
FROM sdtm_variable
    INNER JOIN sdtm_domain ON sdtm_variable.domain_id = sdtm_domain.id
    INNER JOIN variable_type ON sdtm_variable.variable_type_id = variable_type.id
    INNER JOIN variable_role ON sdtm_variable.variable_role_id = variable_role.id
    INNER JOIN variable_core ON sdtm_variable.variable_core_id = variable_core.id;