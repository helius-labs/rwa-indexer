FROM rust:1.74-bullseye
COPY Cargo.toml /rwa-system/Cargo.toml
COPY transformer /rwa-system/transformer
COPY common /rwa-system/common
COPY indexer /rwa-system/indexer
COPY migration /rwa-system/migration
COPY rwa_types /rwa-system/rwa_types
COPY tools /rwa-system/tools
WORKDIR /rwa-system/migration
RUN cargo build --release
CMD /rwa-system/target/release/migration up
