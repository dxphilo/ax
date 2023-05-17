-- Your SQL goes here
CREATE TABLE businesses (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL,
    business VARCHAR NOT NULL,
    business_type VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    selected_amenities TEXT[] NOT NULL,
    images TEXT[] NOT NULL,
    business_name VARCHAR NOT NULL,
    telephone_number VARCHAR NOT NULL,
    business_description VARCHAR NOT NULL,
    days_of_operation TEXT[] NOT NULL,
    opening_hours VARCHAR NOT NULL,
    closing_hours VARCHAR NOT NULL,
    county VARCHAR NOT NULL,
    town VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE reviews (
  id SERIAL NOT NULL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  name VARCHAR NOT  NULL,
  businessid VARCHAR NOT NULL,
  review VARCHAR(140) NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  email VARCHAR(140) NOT NULL,
  image VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL
);