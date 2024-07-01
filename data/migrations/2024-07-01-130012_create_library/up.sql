create table library (
  id serial primary key,
  "name" varchar not null,
  created timestamp default current_timestamp not null,
  modified timestamp default current_timestamp not null
)