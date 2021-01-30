![Rust](https://github.com/beykansen/todo-rs/workflows/Rust/badge.svg)

# ToDo Rust Examle

Simple ToDo app example written in Rust with Actix-Web and MongoDB.


## Usage

Change MongoDB credentials in toml files which are in ``config/`` folder.

(Optional) Provide config file name that will be used in runtime via ``ENV_FILE`` environment variable. Default is ``dev``.

Build and run with ``cargo run`` command.

Access from browser or your favorite api test client.
``
http://localhost:8081
``
## Demos
[Client Demo](https://todo-rs-client.herokuapp.com)

[Api Demo](https://todo-api.beykansen.com)
