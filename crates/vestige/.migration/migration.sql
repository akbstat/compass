-- define table compass_user
CREATE TABLE IF NOT EXISTS compass_user (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN compass_user.name IS 'Unique Name of Compass User';
-- define table history
CREATE TABLE IF NOT EXISTS history (
    id SERIAL PRIMARY KEY,
    product VARCHAR(10) NOT NULL,
    trial VARCHAR(30) NOT NULL,
    purpose VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);
COMMENT ON COLUMN history.product IS 'Product Part of history';
COMMENT ON COLUMN history.trial IS 'Trial Part of history';
COMMENT ON COLUMN history.purpose IS 'Purpose Part of history';
CREATE INDEX IF NOT EXISTS idx_history_unique ON history (product, trial, purpose);
-- define table user_history, many to many of table compass_user and history
CREATE TABLE IF NOT EXISTS user_history (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    history_id INTEGER NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_user_history_unique ON user_history (user_id, history_id);
-- define auto update field updated_at
CREATE OR REPLACE FUNCTION update_timestamp() RETURNS TRIGGER AS $update_timestamp$ BEGIN NEW.updated_at = now();
RETURN NEW;
END;
$update_timestamp$ LANGUAGE plpgsql;
-- binding function update_timestamp to tables which are needed
CREATE OR REPLACE TRIGGER update_compass_user_timestamp BEFORE
UPDATE ON compass_user FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE OR REPLACE TRIGGER update_history_timestamp BEFORE
UPDATE ON history FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- create view user_history_view
CREATE OR REPLACE VIEW user_history_view AS
SELECT user_history.id,
    user_history.user_id,
    history.product,
    history.trial,
    history.purpose,
    user_history.updated_at
FROM user_history
    INNER JOIN history ON user_history.history_id = history.id