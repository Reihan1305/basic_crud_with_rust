FROM rust:1.82.0 as build

# create a new empty shell project
WORKDIR /basic_crud

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
# RUN cargo build --release
# RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY .sqlx .sqlx

# build for release
ENV SQLX_OFFLINE=true
RUN rm -f ./target/release/basic_crud*
RUN cargo build --release

CMD ["/basic_crud/target/release/rust_crud_basic"]

# our final base
# FROM debian:buster-slim

# # copy the build artifact from the build stage
# COPY --from=build /basic_crud/target/release/rust_crud_basic .

# # set the startup command to run your binary
# CMD ["/rust_basic_crud"]