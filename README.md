Tin Fish Index
--------------

There are always more tin fish in the sea, and it has become hard to keep track of them all.
Tin Fish Index is a basic API for the purpose of documenting tin fish products.

## The Project
This project is currently a single API built in Rust with Actix Web.

## Running Locally

To run the application, simply use the `cargo run` command.
This will run and expose the HttpServer at localhost:8080.

If you would like logging, you must configure `env_logger` by setting the `RUST_LOG` environment variable.
For example: `RUST_LOG=info` for info level logging.
