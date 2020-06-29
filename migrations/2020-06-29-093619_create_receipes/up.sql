CREATE TABLE recipes (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL
);

CREATE TABLE instructions (
  id SERIAL PRIMARY KEY,
  recipe_id integer NOT NULL REFERENCES recipes(id),
  step_number integer NOT NULL,
  instruction TEXT NOT NULL
);

CREATE TABLE ingredients (
  id SERIAL PRIMARY KEY,
  recipe_id integer NOT NULL REFERENCES recipes(id),
  ingredient TEXT NOT NULL
);