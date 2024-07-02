create table library_path (
  id serial primary key,
  library_id int not null,
  path varchar not null unique,
  created timestamp default current_timestamp not null,
  modified timestamp default current_timestamp not null,
  constraint fk_library
    foreign key(library_id) references library(id)
    on delete cascade
)