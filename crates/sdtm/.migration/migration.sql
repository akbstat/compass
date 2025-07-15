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
    name VARCHAR(30) NOT NULL,
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
-- define table project
CREATE TABLE IF NOT EXISTS project (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN project.name IS 'Project Name';
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER project_update BEFORE
UPDATE ON project FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table project_version
CREATE TABLE IF NOT EXISTS project_version (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL,
    name VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN project_version.project_id IS 'Project ID';
COMMENT ON COLUMN project_version.name IS 'Project Version Name';
CREATE UNIQUE INDEX IF NOT EXISTS idx_project_version ON project_version (project_id, name);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER project_version_update BEFORE
UPDATE ON project_version FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table form
CREATE TABLE IF NOT EXISTS form (
    id SERIAL PRIMARY KEY,
    version_id INTEGER NOT NULL,
    name VARCHAR(20) NOT NULL,
    description VARCHAR(200) NOT NULL,
    form_order INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN form.version_id IS 'Project Version ID';
COMMENT ON COLUMN form.name IS 'Form Name';
COMMENT ON COLUMN form.description IS 'Form Description';
COMMENT ON COLUMN form.form_order IS 'Form Display Order';
CREATE UNIQUE INDEX IF NOT EXISTS idx_form ON form (version_id, name);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER form_update BEFORE
UPDATE ON form FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table item
CREATE TABLE IF NOT EXISTS item (
    id SERIAL PRIMARY KEY,
    form_id INTEGER NOT NULL,
    name VARCHAR(20) NOT NULL,
    label VARCHAR(1000) NOT NULL,
    item_type_id INTEGER NOT NULL,
    item_order INTEGER NOT NULL,
    item_repeat_index INTEGER NOT NULL,
    item_default_value VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN item.form_id IS 'Form ID';
COMMENT ON COLUMN item.name IS 'Item Name';
COMMENT ON COLUMN item.label IS 'Item Label';
COMMENT ON COLUMN item.item_type_id IS 'Item Type ID';
COMMENT ON COLUMN item.item_order IS 'Item Display Order';
COMMENT ON COLUMN item.item_repeat_index IS 'Item Repeat index in Logline';
COMMENT ON COLUMN item.item_default_value IS 'Item Default Value in Logline';
CREATE UNIQUE INDEX IF NOT EXISTS idx_item ON item (form_id, name, item_repeat_index);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER item_update BEFORE
UPDATE ON item FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table item_type
CREATE TABLE IF NOT EXISTS item_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN item_type.name IS 'Item Type Name';
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER item_type_update BEFORE
UPDATE ON item_type FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table item_option
CREATE TABLE IF NOT EXISTS item_option (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL,
    option_value VARCHAR(200) NOT NULL,
    option_display VARCHAR(200) NOT NULL,
    option_order INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN item_option.item_id IS 'Item ID';
COMMENT ON COLUMN item_option.option_value IS 'Option Actual Value';
COMMENT ON COLUMN item_option.option_display IS 'Option Display Value';
COMMENT ON COLUMN item_option.option_order IS 'Option Display Order';
CREATE UNIQUE INDEX IF NOT EXISTS idx_item_option ON item_option (item_id, option_value);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER item_option_update BEFORE
UPDATE ON item_option FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table item_unit
CREATE TABLE IF NOT EXISTS item_unit (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL,
    name VARCHAR(50) NOT NULL,
    unit_order INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN item_unit.item_id IS 'Item ID';
COMMENT ON COLUMN item_unit.name IS 'Unit Name';
COMMENT ON COLUMN item_unit.unit_order IS 'Unit Display Order';
CREATE UNIQUE INDEX IF NOT EXISTS idx_item_unit ON item_unit (item_id, name);
-- binding function update_timestamp
CREATE OR REPLACE TRIGGER item_unit_update BEFORE
UPDATE ON item_unit FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-----------------------------------------------------------------
------------------ annotation module ----------------------------
-----------------------------------------------------------------
-- define annotation_version
CREATE TABLE IF NOT EXISTS annotation_version (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    project_version_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN annotation_version.name IS 'Annotation Version Name';
COMMENT ON COLUMN annotation_version.project_version_id IS 'Project Version ID';
COMMENT ON COLUMN annotation_version.description IS 'Annotation Version Description';
CREATE UNIQUE INDEX IF NOT EXISTS idx_annotation_version ON annotation_version (project_version_id, name);
CREATE OR REPLACE TRIGGER annotation_version_update BEFORE
UPDATE ON annotation_version FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table form_domain
CREATE TABLE IF NOT EXISTS form_domain (
    id SERIAL PRIMARY KEY,
    annotation_version_id INTEGER NOT NULL,
    form_id INTEGER NOT NULL,
    name VARCHAR(10) NOT NULL,
    description VARCHAR(200) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN form_domain.annotation_version_id IS 'Annotation Version ID';
COMMENT ON COLUMN form_domain.form_id IS 'Form ID';
COMMENT ON COLUMN form_domain.name IS 'Domain Name';
COMMENT ON COLUMN form_domain.description IS 'Domain Description';
CREATE UNIQUE INDEX IF NOT EXISTS idx_form_domain ON form_domain (form_id, annotation_version_id, name);
CREATE OR REPLACE TRIGGER form_domain_update BEFORE
UPDATE ON form_domain FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table form_variable
CREATE TABLE IF NOT EXISTS form_variable (
    id SERIAL PRIMARY KEY,
    domain_id INTEGER NOT NULL,
    name VARCHAR(10),
    supp BOOL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN form_variable.domain_id IS 'Domain ID';
COMMENT ON COLUMN form_variable.name IS 'Variable Name';
COMMENT ON COLUMN form_variable.supp IS 'If Variable is Supplymental or Not, 0 stands for No, 1 stands for Yes';
CREATE UNIQUE INDEX IF NOT EXISTS idx_form_variable ON form_variable (domain_id, name);
CREATE OR REPLACE TRIGGER form_variable_update BEFORE
UPDATE ON form_variable FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- define table annotation
CREATE TABLE IF NOT EXISTS annotation (
    id SERIAL PRIMARY KEY,
    annotation_version_id INTEGER NOT NULL,
    form_id INTEGER NOT NULL,
    variable_id INTEGER NOT NULL,
    source_id INTEGER NOT NULL,
    kind INT NOT NULL,
    annotation_display VARCHAR(1000) NOT NULL,
    assign BOOL NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN annotation.annotation_version_id IS 'Annotation Version ID';
COMMENT ON COLUMN annotation.form_id IS 'Rawdata Form ID';
COMMENT ON COLUMN annotation.variable_id IS 'Form Varaible ID';
COMMENT ON COLUMN annotation.source_id IS 'ID From Form, Item, Item Option or Item Unit';
COMMENT ON COLUMN annotation.kind IS 'Kind of Source ID, 0 stands for Form, 1 stands for Item, 2 stands for Item Value, 3 stands for Item Unit, 4 stands for Item Option';
COMMENT ON COLUMN annotation.annotation_display IS 'Annotation Display Value';
COMMENT ON COLUMN annotation.assign IS 'If annotation is assign or not, 0 stands for not, 1 stands for yes';
CREATE INDEX IF NOT EXISTS idx_annotation ON annotation (form_id, annotation_version_id);
CREATE OR REPLACE TRIGGER annotation_update BEFORE
UPDATE ON annotation FOR EACH ROW EXECUTE FUNCTION update_timestamp();