CREATE TABLE users (
  id VARCHAR PRIMARY KEY,
  email VARCHAR UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL
);

CREATE TABLE recipes (
  id VARCHAR PRIMARY KEY,
  user_id VARCHAR NOT NULL REFERENCES users(id),
  title VARCHAR NOT NULL
);

CREATE TABLE instructions (
  step_number integer NOT NULL,
  recipe_id VARCHAR NOT NULL REFERENCES recipes(id),
  instruction TEXT NOT NULL,
  PRIMARY KEY (recipe_id, step_number)
);

CREATE TABLE ingredients (
  step_number integer NOT NULL,
  recipe_id VARCHAR NOT NULL REFERENCES recipes(id),
  ingredient TEXT NOT NULL,
  PRIMARY KEY (recipe_id, step_number)
);