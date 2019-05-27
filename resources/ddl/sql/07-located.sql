CREATE TABLE rassasy.located (
  id VARCHAR(255) NOT NULL,
  restaurant_id VARCHAR(255) NOT NULL,
  location_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (restaurant_id) REFERENCES restaurant_detail(id),
  FOREIGN KEY (location_id) REFERENCES location(id)
);