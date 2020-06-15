CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  uuid CHAR(36) NOT NULL UNIQUE,
  name VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL
);
