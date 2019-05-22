CREATE TABLE restaurant_detail (
  id VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  city VARCHAR(255),
  state VARCHAR(255),
  description VARCHAR(255),
  notes VARCHAR(255),
  country VARCHAR(255) NOT NULL,
  visited VARCHAR(255) NOT NULL,
  website VARCHAR(255),
  yelp VARCHAR(255),
  PRIMARY KEY (id)
);