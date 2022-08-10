-- Your SQL goes here

CREATE TABLE user_sessions (
	id serial PRIMARY KEY,
	user_id integer NOT NULL REFERENCES users,
	uuid VARCHAR ( 255 ) UNIQUE NOT NULL,
	created_on TIMESTAMP NOT NULL DEFAULT NOW()
);