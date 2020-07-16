ALTER TABLE recipes
ADD COLUMN cook_time_in_minute integer,
ADD COLUMN prep_time_in_minute integer,
ADD COLUMN description VARCHAR,
ADD COLUMN image_url VARCHAR,
ADD COLUMN recipe_yield VARCHAR,
ADD COLUMN category VARCHAR,
ADD COLUMN cuisine VARCHAR,
ADD COLUMN imported_from VARCHAR;
