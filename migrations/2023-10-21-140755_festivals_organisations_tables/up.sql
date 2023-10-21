-- Your SQL goes here


CREATE TABLE organisations (
  id SERIAL PRIMARY KEY,
  name varchar(128) NOT NULL,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  name varchar(128) NOT NULL,
  _types text[],
  organisation_id integer NOT NULL REFERENCES organisations(id),
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE organisation_events (
  id SERIAL PRIMARY KEY,
  organisation_id integer NOT NULL REFERENCES organisations(id),
  event_id integer NOT NULL REFERENCES events(id)
);
