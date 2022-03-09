FROM rust:1.59.0-buster AS build_stage

COPY . /build_dir/

WORKDIR /build_dir/

RUN cargo test \
  && cargo build --release


FROM rust:1.59.0-buster

COPY --from=build_stage /build_dir/target/release/leviathan /usr/local/bin/leviathan

WORKDIR /
ENTRYPOINT ["leviathan"]
