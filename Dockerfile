# Build server
FROM docker.io/library/rust:1.51-buster
WORKDIR /src
COPY . /src
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Build client
FROM docker.io/library/node:15.12-buster
WORKDIR /src
COPY . /src
RUN cd client && \
	yarn install --frozen-lockfile && \
	yarn run generate

# Build resulting image
FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=0 /src/target/x86_64-unknown-linux-gnu/release/social-todo-server /app/
COPY --from=1 /src/client/dist /app/webroot
EXPOSE 8880
ENTRYPOINT ["/app/social-todo-server"]
CMD ["-w", "/app/webroot", "-v", "--no-env"]
