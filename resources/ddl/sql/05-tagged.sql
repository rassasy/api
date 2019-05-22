CREATE TABLE tagged (
  id VARCHAR(255) NOT NULL,
  restaurant_id VARCHAR(255) NOT NULL,
  tag_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (restaurant_id) REFERENCES restaurant_detail(id),
  FOREIGN KEY (tag_id) REFERENCES tag(id)
);