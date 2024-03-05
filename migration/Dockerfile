FROM rust:1.74-bullseye
COPY . /rwa-system/
WORKDIR /rwa-system/migration
RUN cargo build --release
CMD /rwa-system/target/release/migration up
