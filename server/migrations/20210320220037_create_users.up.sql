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
