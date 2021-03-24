# [social-todo-server](https://github.com/vtavernier/social-todo/tree/master/server)

![Server Status](https://github.com/vtavernier/social-todo/workflows/server/badge.svg)

Rust backend using [actix-web](https://actix.rs/) for
[social-todo](https://github.com/vtavernier/social-todo).

## Usage

Note that if a Redis instance is running at `$REDIS_URL` (see its value in
[`.env`](.env)), caching will be enabled on a successful response from the
server. The database server will be hit directly on every request otherwise.

Running the backend server:

```bash
$ cargo run

# Restart the server on changes:
$ cargo watch -x run
```

To run the backend tests:

```bash
$ cargo test

# Re-run tests on changes:
$ cargo watch -x test
```

## Author

Vincent Tavernier <vince.tavernier@gmail.com>
