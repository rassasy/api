CREATE TABLE featured_by (
  id VARCHAR(255) NOT NULL,
  restaurant_id VARCHAR(255) NOT NULL,
  feature_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (feature_id) REFERENCES feature(id),
  FOREIGN KEY (restaurant_id) REFERENCES restaurant_detail(id)
);