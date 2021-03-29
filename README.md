# [social-todo](https://github.com/vtavernier/social-todo)

![Build Status](https://github.com/vtavernier/social-todo/workflows/build/badge.svg)

This repository implements a modern web application and server using a Rust
backend server, and a pre-rendered Vue frontend. This is more of an experiment
with those development tools than a production tool, but still tries to follow
best practices.

## Synopsis

This app is a take on the traditional *to-do list* example, with users
(authentication) and social features, to make for a more interesting challenge.

## Quick start

This assumes you already have:
* A working Rust stable toolchain
* `cargo-watch` installed (`cargo install --force cargo-watch`)
* node.js and the Yarn package manager

```bash
$ cd client
$ yarn install
$ yarn run serve
```

## Repository structure

See the respective READMEs for instructions.

* [`client`](client/): frontend application
* [`server`](server/): backend server

## Continuous integration

### GitHub pages build

See it live at https://vtavernier.github.io/social-todo/.

A static demo is built by the [`build`](.github/workflows/build.yml) workflow.
This version is read-only, as it's destined to be served from GitHub pages (and
thus no backend is available).

The build steps are as follows:
* Setup the database using the schema stored in the repository
* Build and run the backend server
* While the backend server is running, generate the client-side content
* Publish the generated content to GitHub pages

### Heroku build

See it live at https://vt-social-todo.herokuapp.com/.

A read-write version is build by the [`heroku`](.github/workflows/heroku.yml)
workflow. This version can be read-write, as the backend is hosted on a free
Dyno from Heroku, with an attached database and Redis instance.

This relies on the [Container
Registry](https://devcenter.heroku.com/articles/container-registry-and-runtime)
feature of Heroku, which allows us to push pre-built Docker images. In this
case, a multi-stage Dockerfile allows building both the server and client
sides, and packages the result in a
[distroless](https://github.com/GoogleContainerTools/distroless/) based image.

## Author

Vincent Tavernier <vince.tavernier@gmail.com>
