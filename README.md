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

## Author

Vincent Tavernier <vince.tavernier@gmail.com>
