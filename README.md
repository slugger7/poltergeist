# Poltergeist

A media management application that would help you find, manage, convert, cut up, join and manipulate mostly video media.

## Development setup

- Install [rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
- Install [diesel](https://diesel.rs/guides/getting-started)
  - We only need the postgres parts
  - We do need the diesel cli
- Install [docker](https://www.docker.com/)
  - Not necessary but helps with starting the Postgres server

### Database setup

- `docker compose up` to start the database
- `diesel setup` to create the database and run the migrations

### Running application

- In VS Code you can install the rust analyzer (part of the recommended plugins) and go to the [api main file](./api/src/main.rs) and click run
- Or run it with cargo from the root `cargo run api`