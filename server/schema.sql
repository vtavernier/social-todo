CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
	NEW.updated_at = CURRENT_TIMESTAMP;
	RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

DROP TRIGGER IF EXISTS users_update_timestamp ON users;
DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS user_role;
DROP COLLATION IF EXISTS username_collation;

CREATE COLLATION username_collation (
	provider = icu,
	locale = 'und-u-ks-level2',
	deterministic = false
);

CREATE TYPE user_role AS ENUM (
	'user',
	'admin'
);

CREATE TABLE users (
	-- Auto-incremented primary key backed by a Postgres sequence
	id SERIAL PRIMARY KEY,
	-- Username, for login and display
	name VARCHAR(15) NOT NULL
		-- Usernames are case insensitive
		COLLATE username_collation
		-- Usernames must be 4-15 characters, and start with a non-digit
		CHECK ((name COLLATE "C") ~ '^[a-zA-Z_-][a-zA-Z0-9_-]{3,14}$'),
	-- Password, bcrypt hashed
	password CHAR(60) NOT NULL,
	-- Role of the user
	role user_role NOT NULL,
	-- When the user was created
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- When the user was last updated
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- When the user last logged in
	last_login_at TIMESTAMP
);

CREATE TRIGGER users_update_timestamp BEFORE UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE update_timestamp();

-- Insert a sample admin user (with admin password)
INSERT INTO users (id, name, password, role)
VALUES (1, 'Admin', '$2b$12$nWpEhB8I/INUsxS/6TZjVenGudV11Rss/.7xQcPpvNalI.UFUBL62', 'admin');
